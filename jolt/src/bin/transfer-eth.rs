use jolt::Serializable;
use std::time::Instant;
use utils::{benchmark, BenchResult, ETHTRANSFER_INPUTS};

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/ethtransfer_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(benchmark_transfer_eth, &ETHTRANSFER_INPUTS, &csv_file);
}

fn benchmark_transfer_eth(n: usize) -> BenchResult {
    let (prove_fn, _verify_fn) = transfer_eth_guest::build_transfer_eth_n_times();
    let program_summary = transfer_eth_guest::analyze_transfer_eth_n_times(n);

    let start = Instant::now();
    let (_output, proof) = prove_fn(n);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
