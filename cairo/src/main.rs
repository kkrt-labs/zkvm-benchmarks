use utils::{
    bench::{benchmark, Metrics},
    metadata::{FIBONACCI_INPUTS, SHA2_INPUTS},
};

use cairo_air::verifier::verify_cairo;
use cairo_air::PreProcessedTraceVariant;
use cairo_lang_runner::Arg;
use cairo_prove::execute::execute;
use cairo_prove::prove::{prove, prover_input_from_runner};
use cairo_vm::Felt252;
use sonic_rs;
use std::env;
use stwo_cairo_prover::stwo_prover::core::fri::FriConfig;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

/// Configurations for the CSTARK prover.
///
/// Conjecture of n-bit security level: `n = n_queries * log_blowup_factor + pow_bits`.
/// Configuration to achieve 96-bit security level, with PoW bits inferior to 20.
///
/// - The blowup factor greatly influences the proving time.
/// - The number of queries influences the proof size.
/// - The PoW bits influence the proving time, depending on the hardware and the number of bits to grind.
pub const REGULAR_96_BITS: PcsConfig = PcsConfig {
    pow_bits: 16,
    fri_config: FriConfig {
        log_last_layer_degree_bound: 0,
        log_blowup_factor: 1,
        n_queries: 80,
    },
};

fn bench_cairo_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);
    let target_path = "test_data/target/release/fib.executable.json";
    let args = vec![Arg::Value(Felt252::from(n))];

    let pcs_config = REGULAR_96_BITS;

    // Execute.
    let start_time = std::time::Instant::now();
    let executable =
        sonic_rs::from_reader(std::fs::File::open(target_path).expect("Failed to open executable"))
            .expect("Failed to read executable");
    let runner = execute(executable, args);
    metrics.exec_duration = start_time.elapsed();

    // Prove.
    let start_time = std::time::Instant::now();
    let prover_input = prover_input_from_runner(&runner);
    let proof = prove(prover_input, pcs_config);
    metrics.proof_duration = start_time.elapsed();
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // Verify.
    let start_time = std::time::Instant::now();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
    assert!(result.is_ok());
    metrics.verify_duration = start_time.elapsed();

    metrics
}

fn bench_cairo_sha256(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);
    let target_path = "test_data/target/release/sha256.executable.json";

    // Pass the size as a single felt252 argument
    let args = vec![Arg::Value(Felt252::from(n))];

    let pcs_config = REGULAR_96_BITS;

    // Execute.
    let start_time = std::time::Instant::now();
    let executable =
        sonic_rs::from_reader(std::fs::File::open(target_path).expect("Failed to open executable"))
            .expect("Failed to read executable");
    let runner = execute(executable, args);
    metrics.exec_duration = start_time.elapsed();

    // Prove.
    let start_time = std::time::Instant::now();
    let prover_input = prover_input_from_runner(&runner);
    let proof = prove(prover_input, pcs_config);
    metrics.proof_duration = start_time.elapsed();
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // Verify.
    let start_time = std::time::Instant::now();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
    assert!(result.is_ok());
    metrics.verify_duration = start_time.elapsed();

    metrics
}

/// Runs a compiled Cairo program and generate a proof of execution.
fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <fib|sha256>", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "fib" => {
            benchmark(
                bench_cairo_fib,
                &FIBONACCI_INPUTS,
                "../.outputs/benchmark/fib_cairo.csv",
            );
        }
        "sha256" => {
            let sha256_inputs: Vec<u32> = SHA2_INPUTS.iter().map(|&x| x as u32).collect();
            benchmark(
                bench_cairo_sha256,
                &sha256_inputs,
                "../.outputs/benchmark/sha2_cairo.csv",
            );
        }
        _ => {
            eprintln!("Invalid benchmark type: {}. Use 'fib' or 'sha256'", args[1]);
            std::process::exit(1);
        }
    }
}
