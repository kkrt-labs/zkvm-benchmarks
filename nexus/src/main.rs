use std::{
    time::{Duration, Instant},
    usize,
};
use utils::benchmark;

use k256::{ecdsa::Signature, elliptic_curve::sec1::EncodedPoint, Secp256k1};

use nexus_sdk::{
    compile::CompileOpts,
    nova::seq::{Generate, Nova, PP},
    Local, Prover, Verifiable,
};

const FIB_PACKAGE: &str = "fibonacci-guest";
const SHA2_PACKAGE: &str = "sha2-guest";
const ECDSA_PACKAGE: &str = "ecdsa-guest";
const TRANSFER_ETH_PACKAGE: &str = "transfer-eth-guest";

const MESSAGE: &[u8] = include_bytes!("../../helper/ecdsa_signature/message.txt");
const KEY: &[u8] = include_bytes!("../../helper/ecdsa_signature/verifying_key.txt");
const SIGNATURE: &[u8] = include_bytes!("../../helper/ecdsa_signature/signature.txt");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--once") {
        once_fib();
    } else {
        let ns = [10, 100];
        benchmark(
            benchmark_fib,
            &ns,
            "../benchmark_outputs/fib_nexus.csv",
            "n",
        );

        // println!("==========Starting SHA2...");

        // let lengths = [32, 256];
        // benchmark(benchmark_sha2, &lengths, "../benchmark_outputs/sha2_nexus.csv", "n");

        // println!("==========Starting ECDSA...");
        // let lengths = [1];
        // benchmark(
        //     benchmark_ecdsa_verify,
        //     &lengths,
        //     "../benchmark_outputs/ecdsa_nexus.csv",
        //     "n",
        // );

        // println!("==========Starting ETHTransfer...");
        // let lengths = [1, 10];
        // benchmark(
        //     benchmark_transfer_eth,
        //     &lengths,
        //     "../benchmark_outputs/ethtransfer_nexus.csv",
        //     "n",
        // );
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
    let _proof = prover
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

    let message = hex::decode(MESSAGE).expect("Failed to decode hex of 'message'");

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        &hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let start = Instant::now();
    let proof = prover
        .prove_with_input::<(EncodedPoint<Secp256k1>, Vec<u8>, Signature)>(
            &pp,
            &(encoded_point, message, signature),
        )
        .expect("failed to prove program");
    let end = Instant::now();
    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");

    (end.duration_since(start), 0x1000000, 0x1000000)
}

fn benchmark_transfer_eth(n: usize) -> (Duration, usize, usize) {
    println!("Setting up Nova public parameters...");
    let pp: PP = PP::generate().expect("failed to generate parameters");

    let mut opts = CompileOpts::new(TRANSFER_ETH_PACKAGE);
    opts.set_memlimit(8); // use an 8mb memory

    println!("Compiling guest program...");
    let prover: Nova<Local> = Nova::compile(&opts).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    let start = Instant::now();
    let proof = prover
        .prove_with_input::<usize>(&pp, &n)
        .expect("failed to prove program");
    let end = Instant::now();
    println!(">>>>> Logging\n{}<<<<<", proof.logs().join(""));

    print!("Verifying execution...");
    proof.verify(&pp).expect("failed to verify proof");

    println!("  Succeeded!");

    (end.duration_since(start), 0x1000000, 0x1000000)
}
