use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{bench::Metrics, bench::benchmark, load_elf, size};

fn main() {
    let lengths = [1];
    benchmark(
        bench_ethblock,
        &lengths,
        "../.outputs/benchmark/ethblock_pico.csv",
    );
}

fn bench_ethblock(num_txs: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(num_txs);

    init_logger();
    let elf = load_elf("./ethblock-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&num_txs);

    let now = Instant::now();
    let proof = client.prove_fast().expect("Failed to generate proof");
    metrics.proof_duration = now.elapsed();
    metrics.proof_bytes = size(&proof.proofs);

    metrics
}
