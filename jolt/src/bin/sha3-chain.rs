use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

type BenchResult = (Duration, usize, usize);

fn main() {
    // let iters = [230, 460, 920, 1840, 3680];
    let iters = [230, 250];
    benchmark(benchmark_sha3_chain, &iters, "../benchmark_outputs/sha3_chain_jolt.csv", "iters");
}

fn benchmark_sha3_chain(iters: u32) -> BenchResult {
    let (prove_sha3_chain, _verify_sha3_chain) = sha3_chain_guest::build_sha3_chain();
    let input = [5u8; 32];
    let program_summary = sha3_chain_guest::analyze_sha3_chain(input, iters);

    let start = Instant::now();
    let (_output, proof) = prove_sha3_chain(input, iters);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}