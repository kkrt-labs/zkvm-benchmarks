use utils::{bench::benchmark, metadata::ETHTRANSFER_INPUTS};
use zkm_script::{bench_ethtransfer, init_logger};

fn main() {
    init_logger();

    benchmark(
        bench_ethtransfer,
        &ETHTRANSFER_INPUTS,
        "../.outputs/benchmark/ethtransfer_zkm.csv",
    );
}
