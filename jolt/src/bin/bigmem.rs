use std::time::{Duration, Instant};
use jolt::Serializable;
use utils::benchmark;

type BenchResult = (Duration, usize, usize);

fn main() {
    let csv_file = format!(
        "../benchmark_outputs/bigmem_jolt{}{}.csv",
        if cfg!(feature = "icicle") { "-gpu" } else { "" },
        ""
    );

    let values = [5];
    benchmark(benchmark_bigmem, &values, &csv_file, "value");
}

fn benchmark_bigmem(value: u32) -> BenchResult {
    let (prove_bigmem, verify_bigmem) = bigmem_guest::build_waste_memory();
    let program_summary = bigmem_guest::analyze_waste_memory(value);

    let start = Instant::now();
    let (_output, proof) = prove_bigmem(value);
    let end = Instant::now();

    (
        end.duration_since(start),
        proof.size().unwrap(),
        program_summary.processed_trace.len(),
    )
}
