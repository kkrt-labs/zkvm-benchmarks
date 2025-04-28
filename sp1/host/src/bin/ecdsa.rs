//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```
//!

use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use std::time::Instant;
use utils::{bench::benchmark, ecdsa_input, metadata::ECDSA_INPUTS, size, bench::BenchResult};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ECDSA_ELF: &[u8] = include_elf!("ecdsa-guest");

fn main() {
    if std::env::var("SP1_PROVER").unwrap_or_default() == "cuda" {
        benchmark(
            bench_ecdsa,
            &ECDSA_INPUTS,
            "../benchmark_outputs/ecdsa_sp1turbo-gpu.csv",
        );
    } else {
        benchmark(
            bench_ecdsa,
            &ECDSA_INPUTS,
            "../benchmark_outputs/ecdsa_sp1turbo.csv",
        );
    }
}

fn bench_ecdsa(_dummy: usize) -> BenchResult {
    let input = ecdsa_input();
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&input);

    // Execute the program
    let (_, report) = client.execute(ECDSA_ELF, &stdin).run().unwrap();
    println!("Program executed successfully.");

    // Record the number of cycles executed.
    println!("Number of cycles: {}", report.total_instruction_count());

    // Setup the program for proving.
    let (pk, vk) = client.setup(ECDSA_ELF);

    let start = Instant::now();
    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");
    let end = Instant::now();
    let duration = end.duration_since(start);

    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");

    (
        duration,
        size(&proof),
        report.total_instruction_count() as usize,
    )
}
