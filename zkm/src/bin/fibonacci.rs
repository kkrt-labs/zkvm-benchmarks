use utils::{bench::benchmark, metadata::FIBONACCI_INPUTS};
use zkm_script::{bench_fibonacci, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_fibonacci,
        &FIBONACCI_INPUTS,
        "../benchmark_outputs/fib_zkm.csv",
    );
}
