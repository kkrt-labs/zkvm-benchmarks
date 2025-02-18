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
const ECDSA_PACKAGE: &str = "ecdsa-guest";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--once") {
        once_fib();
    } else {
        // let lengths = [1];
        // benchmark(
        //     benchmark_ecdsa_verify,
        //     &lengths,
        //     "../benchmark_outputs/ecdsa_nexus.csv",
        //     "byte length",
        // );

        // let ns = [10, 50, 90];
        // benchmark(
        //     benchmark_fib,
        //     &ns,
        //     "../benchmark_outputs/fib_nexus.csv",
        //     "n",
        // );

        let lengths = [32, 256, 512, 1024];
        benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_nexus.csv", "byte length");
    }
}

fn once_fib() {
    let n = 100u32;
    println!("Profile mode activated: executing bench_fib(100) only...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    let mut opts = CompileOpts::new(FIB_PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let proof = prover
        .prove_with_input::<u32>(&pp, &n)
        .expect("failed to prove program");
    println!("  Succeeded!");
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

fn benchmark_ecdsa_verify(_length: usize) -> (Duration, usize, usize) {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    let mut opts = CompileOpts::new(ECDSA_PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let start = Instant::now();
    let proof = prover
        .prove_with_input::<()>(&pp, &())
        .expect("failed to prove program");
    let end = Instant::now();
    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");

    (end.duration_since(start), 0x1000000, 0x1000000)
}
