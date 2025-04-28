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
use utils::{bench::benchmark, metadata::ETHTRANSFER_INPUTS, size, bench::BenchResult};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const EVM_ELF: &[u8] = include_elf!("transfer-eth-guest");

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
        benchmark(
            bench_evm,
            &[n],
            format!("../benchmark_outputs/ethtransfer_sp1turbo-gpu-{}.csv", n).as_str(),
        );
    } else {
        benchmark(
            bench_evm,
            &ETHTRANSFER_INPUTS,
            "../benchmark_outputs/ethtransfer_sp1turbo.csv",
        );
    }
}

fn bench_evm(num_txs: usize) -> BenchResult {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&num_txs);

    // Execute the program
    let (_, report) = client.execute(EVM_ELF, &stdin).run().unwrap();
    println!("Program executed successfully.");

    // Record the number of cycles executed.
    println!("Number of cycles: {}", report.total_instruction_count());

    // Setup the program for proving.
    let (pk, vk) = client.setup(EVM_ELF);

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
