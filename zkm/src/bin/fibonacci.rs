use utils::{bench::benchmark_v2, metadata::FIBONACCI_INPUTS};
use zkm_script::{bench_fibonacci, init_logger};

fn main() {
    init_logger();

    benchmark_v2(
        bench_fibonacci,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_zkm.csv",
    );
}
