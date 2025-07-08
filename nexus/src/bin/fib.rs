use nexus_sdk::{
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
    compile::{Compile, Compiler, cargo::CargoPackager},
    stwo::seq::Stwo,
};
use std::time::Instant;
use utils::{bench::Metrics, bench::benchmark, metadata::FIBONACCI_INPUTS, size};

const PACKAGE: &str = "fibonacci-guest";

fn main() {
    benchmark(
        benchmark_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_nexus.csv",
    );
}

fn benchmark_fib(n: u32) -> Metrics {
    let mut metrics: Metrics = Metrics::new(n as usize);

    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with verification

    let start = Instant::now();
    let _ = prover
        .run_with_input::<u32, ()>(&n, &())
        .expect("failed to run program");
    metrics.exec_duration = start.elapsed();

    let start = Instant::now();
    let (view, proof) = prover
        .prove_with_input::<u32, ()>(&n, &())
        .expect("failed to prove program");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = size(&proof);

    let output = view
        .public_output::<u128>()
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
