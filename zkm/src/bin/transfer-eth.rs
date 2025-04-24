use utils::{benchmark, ETHTRANSFER_INPUTS};
use zkm_script::{bench_ethtransfer, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_ethtransfer,
        &ETHTRANSFER_INPUTS,
        "../benchmark_outputs/ethtransfer_zkm.csv",
    );
}
