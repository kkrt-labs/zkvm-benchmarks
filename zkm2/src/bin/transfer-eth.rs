use utils::benchmark;
use zkm2_script::{bench_transfer_eth, init_logger};

fn main() {
    init_logger();

    let ns = [10, 50, 90];
    benchmark(bench_transfer_eth, &ns, "../benchmark_outputs/transfer_eth_zkm2.csv", "num_transfers");
}
