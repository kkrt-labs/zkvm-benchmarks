use nexus_sdk::{
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
    compile::{Compile, Compiler, cargo::CargoPackager},
    stwo::seq::Stwo,
};
use std::time::Instant;
use utils::{bench::Metrics, bench::benchmark, metadata::SHA2_INPUTS, sha2_input, size};

const PACKAGE: &str = "sha2-guest";

fn main() {
    benchmark(
        benchmark_sha2,
        &SHA2_INPUTS,
        "../.outputs/benchmark/sha2_nexus.csv",
    );
}

fn benchmark_sha2(num_bytes: usize) -> Metrics {
    let mut metrics: Metrics = Metrics::new(num_bytes as usize);
    let input = sha2_input(num_bytes);

    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with verification

    let start = Instant::now();
    let _ = prover
        .run_with_input::<Vec<u8>, ()>(&input, &())
        .expect("failed to run program");
    metrics.exec_duration = start.elapsed();
    metrics.elf_line_counts = elf.instructions.len() as u64;

    let start = Instant::now();
    let (view, proof) = prover
        .prove_with_input::<Vec<u8>, ()>(&input, &())
        .expect("failed to prove program");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = size(&proof);

    let output = view
        .public_output::<[u8; 32]>()
        .expect("failed to retrieve public output");

    let start = Instant::now();
    proof
        .verify_expected(
            &(), // no public input
            nexus_sdk::KnownExitCodes::ExitSuccess as u32,
            &output, // no public output
            &elf,    // expected elf (program binary)
            &[],     // no associated data,
        )
        .expect("failed to verify proof");
    metrics.verify_duration = start.elapsed();

    metrics
}
