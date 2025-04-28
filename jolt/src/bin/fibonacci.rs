use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark_v2, bench::Metrics, metadata::FIBONACCI_INPUTS};

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/fib_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark_v2(benchmark_fib, &FIBONACCI_INPUTS, &csv_file);
}

fn benchmark_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);
    let (prove_fib, verify_fib) = fibonacci_guest::build_fib();

    let start = Instant::now();
    let program_summary = fibonacci_guest::analyze_fib(n);
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;

    let start = Instant::now();
    let (_output, proof) = prove_fib(n);
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verify_fib(proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
