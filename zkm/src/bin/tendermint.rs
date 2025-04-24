use utils::benchmark;
use zkm_script::{bench_tendermint, init_logger};

fn main() {
    init_logger();

    let values = [2279100u32];
    benchmark(
        bench_tendermint,
        &values,
        "../benchmark_outputs/tendermint_zkm.csv",
    );
}
