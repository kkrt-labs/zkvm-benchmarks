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
use utils::{bench::benchmark_v2, bench::Metrics, ecdsa_input, metadata::ECDSA_INPUTS, size};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ECDSA_ELF: &[u8] = include_elf!("ecdsa-guest");

fn main() {
    if std::env::var("SP1_PROVER").unwrap_or_default() == "cuda" {
        benchmark_v2(
            bench_ecdsa,
            &ECDSA_INPUTS,
            "../.outputs/benchmark/ecdsa_sp1turbo-gpu.csv",
        );
    } else {
        benchmark_v2(
            bench_ecdsa,
            &ECDSA_INPUTS,
            "../.outputs/benchmark/ecdsa_sp1turbo.csv",
        );
    }
}

fn bench_ecdsa(n: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(n);

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
    let start = Instant::now();
    let (_, report) = client.execute(ECDSA_ELF, &stdin).run().unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.cycles = report.total_instruction_count() as u64;

    // Setup the program for proving.
    let (pk, vk) = client.setup(ECDSA_ELF);

    let start = Instant::now();
    // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = size(&proof);

    // Verify the proof.
    let start = Instant::now();
    client.verify(&proof, &vk).expect("failed to verify proof");
    metrics.verify_duration = start.elapsed();

    metrics
}
