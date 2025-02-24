use utils::benchmark;
use zkm2_script::{bench_fibonacci, init_logger};

fn main() {
    init_logger();

    let ns = [10, 50, 90];
    benchmark(bench_fibonacci, &ns, "../benchmark_outputs/fibonacci_zkm2.csv", "n");
}
