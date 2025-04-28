use pico_sdk::{client::DefaultProverClient, init_logger};
use std::time::Instant;
use utils::{
    bench::Metrics, bench::benchmark_v2, ecdsa_input, load_elf, metadata::ECDSA_INPUTS, size,
};

fn main() {
    benchmark_v2(
        bench_ecdsa,
        &ECDSA_INPUTS,
        "../.outputs/benchmark/ecdsa_pico.csv",
    );
}

fn bench_ecdsa(n: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(n as usize);

    init_logger();
    let elf = load_elf("./ecdsa-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let input = ecdsa_input();
    stdin_builder.borrow_mut().write(&input);

    let now = Instant::now();
    let proof = client.prove_fast().expect("Failed to generate proof");
    metrics.proof_duration = now.elapsed();
    metrics.proof_bytes = size(&proof.proofs);

    metrics
}
