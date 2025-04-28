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
use utils::{bench::benchmark_v2, bench::Metrics, metadata::SHA2_INPUTS, sha2_input, size};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA2_ELF: &[u8] = include_elf!("sha2-guest");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if std::env::var("SP1_PROVER").unwrap_or_default() == "cuda" {
        let n: usize = args
            .iter()
            .skip_while(|arg| *arg != "--n")
            .nth(1)
            .expect("Please provide a value for --n")
            .parse()
            .expect("Value for --n should be a valid u32");
        benchmark_v2(
            bench_sha2,
            &[n],
            format!("../.outputs/benchmark/sha2_sp1turbo-gpu-{}.csv", n).as_str(),
        );
    } else {
        benchmark_v2(
            bench_sha2,
            &SHA2_INPUTS,
            "../.outputs/benchmark/sha2_sp1turbo.csv",
        );
    }
}

fn bench_sha2(num_bytes: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(num_bytes as usize);

    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    let input = sha2_input(num_bytes);
    stdin.write(&input);

    // Execute the program
    let start = Instant::now();
    let (_, report) = client.execute(SHA2_ELF, &stdin).run().unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.cycles = report.total_instruction_count() as u64;

    // Setup the program for proving.
    let (pk, vk) = client.setup(SHA2_ELF);

    let start = Instant::now();
    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = size(&proof);

    println!("Successfully generated proof!");

    // Verify the proof.
    let start = Instant::now();
    client.verify(&proof, &vk).expect("failed to verify proof");
    metrics.verify_duration = start.elapsed();

    metrics
}
