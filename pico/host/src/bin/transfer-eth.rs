use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{bench::Metrics, bench::benchmark, load_elf, metadata::ETHTRANSFER_INPUTS, size};

fn main() {
    benchmark(
        bench_transfer_eth,
        &ETHTRANSFER_INPUTS,
        "../.outputs/benchmark/ethtransfer_pico.csv",
    );
}

fn bench_transfer_eth(n: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(n as usize);

    init_logger();
    let elf = load_elf("./transfer-eth-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&n);

    let now = Instant::now();
    let proof = client.prove_fast().expect("Failed to generate proof");
    metrics.proof_duration = now.elapsed();
    metrics.proof_bytes = size(&proof.proofs);

    metrics
}
