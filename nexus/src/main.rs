use std::{
    time::{Duration, Instant},
    usize,
};
use utils::benchmark;

use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};

const FIB_PACKAGE: &str = "fibonacci-guest";
const SHA2_PACKAGE: &str = "sha2-guest";

fn main() {
    let ns = [32, 64];
    benchmark(
        benchmark_fib,
        &ns,
        "../benchmark_outputs/fib_nexus.csv",
        "n",
    );

    let lengths = [32, 256, 512, 1024, 2048];
    benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_nexus.csv", "byte length");
}

fn benchmark_fib(n: u32) -> (Duration, usize, usize) {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    let mut opts = CompileOpts::new(FIB_PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let start = Instant::now();
    let proof = prover
        .prove_with_input::<u32>(&pp, &n)
        .expect("failed to prove program");
    let end = Instant::now();
    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");

    (end.duration_since(start), 0x1000000, 0x1000000)
}

fn benchmark_sha2(num_bytes: usize) -> (Duration, usize, usize) {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");
    let input = vec![5u8; num_bytes];

    let mut opts = CompileOpts::new(SHA2_PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let start = Instant::now();
    let proof = prover
        .prove_with_input::<Vec<u8>>(&pp, &input)
        .expect("failed to prove program");
    let end = Instant::now();
    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");

    (end.duration_since(start), 0x1000000, 0x1000000)
}
