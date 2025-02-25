use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

type BenchResult = (Duration, usize, usize);

fn main() {
    let ns = [10, 50, 90];
    let csv_file = format!(
        "../benchmark_outputs/fib_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(
        benchmark_fib,
        &ns,
        &csv_file,
        "n",
    );
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
