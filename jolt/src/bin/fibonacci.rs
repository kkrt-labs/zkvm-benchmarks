use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark, bench::Metrics, metadata::FIBONACCI_INPUTS};

const TARGET_DIR: &str = "./fibonacci-guest";

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/fib_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(benchmark_fib, &FIBONACCI_INPUTS, &csv_file);
}

fn benchmark_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    let program = fibonacci_guest::compile_fib(TARGET_DIR);
    let prover_preprocessing = fibonacci_guest::preprocess_prover_fib(&program);
    let verifier_preprocessing = fibonacci_guest::preprocess_verifier_fib(&program);

    let prover = fibonacci_guest::build_prover_fib(program, prover_preprocessing);
    let verifier = fibonacci_guest::build_verifier_fib(verifier_preprocessing);

    let start = Instant::now();
    let program_summary = fibonacci_guest::analyze_fib(n);
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;

    let start = Instant::now();
    let (output, proof) = prover(n);
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verifier(n, output, proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
