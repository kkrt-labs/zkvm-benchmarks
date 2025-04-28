use jolt::Serializable;
use std::time::Instant;
use utils::{bench::benchmark, bench::Metrics, metadata::ETHTRANSFER_INPUTS};

fn main() {
    let csv_file = format!(
        "../.outputs/benchmark/ethtransfer_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    benchmark(benchmark_transfer_eth, &ETHTRANSFER_INPUTS, &csv_file);
}

fn benchmark_transfer_eth(n: usize) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    let (prove_fn, verify_fn) = transfer_eth_guest::build_transfer_eth_n_times();

    let start = Instant::now();
    let program_summary = transfer_eth_guest::analyze_transfer_eth_n_times(n);
    metrics.exec_duration = start.elapsed();
    metrics.cycles = program_summary.processed_trace.len() as u64;

    let start = Instant::now();
    let (_output, proof) = prove_fn(n);
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.size().unwrap();

    let start = Instant::now();
    let _verify_result = verify_fn(proof);
    metrics.verify_duration = start.elapsed();

    metrics
}
