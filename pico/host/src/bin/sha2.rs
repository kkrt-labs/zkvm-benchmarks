use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{bench::Metrics, bench::benchmark, load_elf, metadata::SHA2_INPUTS, sha2_input, size};

fn main() {
    benchmark(
        bench_hash,
        &SHA2_INPUTS,
        "../.outputs/benchmark/sha2_pico.csv",
    );
}

fn bench_hash(num_bytes: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(num_bytes as usize);

    init_logger();
    let elf = load_elf("./sha2-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let input = sha2_input(num_bytes);
    stdin_builder.borrow_mut().write(&input);

    let now = Instant::now();
    let proof = client.prove_fast().expect("Failed to generate proof");
    metrics.proof_duration = now.elapsed();
    metrics.proof_bytes = size(&proof.proofs);

    metrics
}
