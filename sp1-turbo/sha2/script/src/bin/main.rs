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
use std::time::Duration;
use utils::{benchmark, size};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA2_ELF: &[u8] = include_elf!("sha2-program");
type BenchResult = (Duration, usize, usize);

fn main() {
    let lengths = [32, 256, 512, 1024];
    benchmark(
        bench_sha2,
        &lengths,
        "../benchmark_outputs/sha2_sp1turbo.csv",
        "byte length",
    );
}

fn bench_sha2(num_bytes: usize) -> BenchResult {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    let input = vec![5u8; num_bytes];
    stdin.write(&input);

    // Execute the program
    let (_, report) = client.execute(SHA2_ELF, &stdin).run().unwrap();
    println!("Program executed successfully.");

    // Record the number of cycles executed.
    println!("Number of cycles: {}", report.total_instruction_count());

    // Setup the program for proving.
    let (pk, vk) = client.setup(SHA2_ELF);

    let start = std::time::Instant::now();
    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");
    let end = std::time::Instant::now();
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
