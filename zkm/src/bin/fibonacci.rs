use utils::benchmark;
use zkm_script::{benchmark_fibonacci, init_logger};

fn main() {
    init_logger();

    let ns = [10, 50, 90];
    benchmark(benchmark_fibonacci, &ns, "../benchmark_outputs/fib_zkm.csv", "n");
}
