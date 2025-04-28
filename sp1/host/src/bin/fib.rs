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
use utils::{bench::benchmark, bench::Metrics, metadata::FIBONACCI_INPUTS, size};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci-guest");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if std::env::var("SP1_PROVER").unwrap_or_default() == "cuda" {
        let n: u32 = args
            .iter()
            .skip_while(|arg| *arg != "--n")
            .nth(1)
            .expect("Please provide a value for --n")
            .parse()
            .expect("Value for --n should be a valid u32");

        benchmark(
            bench_fib,
            &[n],
            format!("../.outputs/benchmark/fib_sp1-gpu-{}.csv", n).as_str(),
        );
    } else {
        benchmark(
            bench_fib,
            &FIBONACCI_INPUTS,
            "../.outputs/benchmark/fib_sp1.csv",
        );
    }
}

fn bench_fib(n: u32) -> Metrics {
    let mut metrics: Metrics = Metrics::new(n as usize);

    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Execute the program
    let start = Instant::now();
    let (_output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.cycles = report.total_instruction_count();

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

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
