use utils::{bench::benchmark, metadata::FIBONACCI_INPUTS};
use zkm_script::{bench_zkm_fib, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_zkm_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_zkm.csv",
    );
}
