use std::time::Instant;
use utils::bench::{benchmark, Metrics};
use utils::metadata::FIBONACCI_INPUTS;
use stark_v_sdk::prover::{prove_rv32im, verify_rv32im, PcsConfig, FriConfig};
use stark_v_sdk::runner;

const GUEST_ELF: &[u8] = include_bytes!(
    "../../guest/target/riscv32im-unknown-none-elf/release/stark-v-fib-guest"
);

/// Same as stwo-cairo: 96-bit security
/// Formula: n = n_queries * log_blowup_factor + pow_bits
/// 96 = 80 * 1 + 16
const SECURITY_96_BITS: PcsConfig = PcsConfig {
    pow_bits: 16,
    fri_config: FriConfig {
        log_last_layer_degree_bound: 0,
        log_blowup_factor: 1,
        n_queries: 80,
    },
};

fn bench_stark_v_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);
    let input = n.to_le_bytes();

    // Execute
    let start = Instant::now();
    let run_result = runner::run_with_input(GUEST_ELF, &input, u64::MAX).unwrap();
    metrics.exec_duration = start.elapsed();

    // Prove with 96-bit security (matching stwo-cairo)
    let start = Instant::now();
    let proof = prove_rv32im(run_result, SECURITY_96_BITS);
    metrics.proof_duration = start.elapsed();
    // Use size_estimate() like cairo benchmark
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // Verify
    let start = Instant::now();
    verify_rv32im(proof, SECURITY_96_BITS).unwrap();
    metrics.verify_duration = start.elapsed();

    metrics
}

fn main() {
    benchmark(bench_stark_v_fib, &FIBONACCI_INPUTS, "../.outputs/benchmark/fib_stark-v.csv");
}
