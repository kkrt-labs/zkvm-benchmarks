use utils::benchmark;
use zkm_script::{benchmark_transfer_eth, init_logger};

fn main() {
    init_logger();

    let ns = [1, 10, 100];
    benchmark(benchmark_transfer_eth, &ns, "../benchmark_outputs/ethtransfer_zkm.csv", "n");
}
