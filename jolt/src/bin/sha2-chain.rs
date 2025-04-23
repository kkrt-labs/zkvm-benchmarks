use jolt::Serializable;
use std::time::Instant;
use utils::{benchmark, BenchResult};

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/sha2_chain_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );
    // let iters = [230, 460, 920, 1840, 3680];
    let iters = [230, 250];
    benchmark(benchmark_sha2_chain, &iters, &csv_file);
}

fn benchmark_sha2_chain(iters: u32) -> BenchResult {
    let (prove_sha2_chain, _verify_sha2_chain) = sha2_chain_guest::build_sha2_chain();
    let input = [5u8; 32];
    let program_summary = sha2_chain_guest::analyze_sha2_chain(input, iters);

    let start = Instant::now();
    let (_output, proof) = prove_sha2_chain(input, iters);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
