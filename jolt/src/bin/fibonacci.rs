use jolt::Serializable;
use std::time::Instant;
use utils::{benchmark, BenchResult, FIBONACCI_INPUTS};

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/fib_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(benchmark_fib, &FIBONACCI_INPUTS, &csv_file);
}

fn benchmark_fib(n: u32) -> BenchResult {
    let (prove_fib, _verify_fib) = fibonacci_guest::build_fib();
    let program_summary = fibonacci_guest::analyze_fib(n);

    let start = Instant::now();
    let (_output, proof) = prove_fib(n);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
