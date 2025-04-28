use nexus_sdk::{
    ByGuestCompilation, Local, Prover, Verifiable, Viewable,
    compile::{Compile, Compiler, cargo::CargoPackager},
    stwo::seq::Stwo,
};
use std::time::Instant;
use utils::{bench::BenchResult, bench::benchmark, metadata::FIBONACCI_INPUTS, size};

const PACKAGE: &str = "fibonacci-guest";

fn main() {
    benchmark(
        benchmark_fib,
        &FIBONACCI_INPUTS,
        "../benchmark_outputs/fib_nexus.csv",
    );
}

fn benchmark_fib(n: u32) -> BenchResult {
    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    let elf = prover.elf.clone(); // save elf for use with verification

    println!("Proving execution of vm...");
    let start = Instant::now();
    let (view, proof) = prover
        .prove_with_input::<u32, ()>(&n, &())
        .expect("failed to prove program");
    let end = Instant::now();

    println!(
        ">>>>> Logging\n{}<<<<<",
        view.logs().expect("failed to retrieve debug logs").join("")
    );
    assert_eq!(
        view.exit_code().expect("failed to retrieve exit code"),
        nexus_sdk::KnownExitCodes::ExitSuccess as u32
    );

    print!("Verifying execution...");

    let output = view
        .public_output::<u128>()
        .expect("failed to retrieve public output");

    #[rustfmt::skip]
    proof
        .verify_expected(
            &(),  // no public input
            nexus_sdk::KnownExitCodes::ExitSuccess as u32,
            &output,  // no public output
            &elf, // expected elf (program binary)
            &[],  // no associated data,
        )
        .expect("failed to verify proof");

    println!("  Succeeded!");

    (end.duration_since(start), size(&proof), 0x0)
}
