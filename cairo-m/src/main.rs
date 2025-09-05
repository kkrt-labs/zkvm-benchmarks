use cairo_m_common::InputValue;
use cairo_m_compiler::{compile_cairo, CompilerOptions};
use cairo_m_prover::{
    adapter::import_from_runner_output, prover::prove_cairo_m, prover_config::REGULAR_96_BITS,
    verifier::verify_cairo_m,
};
use cairo_m_runner::run_cairo_program;
use sha2::{Digest, Sha256};
use std::convert::TryInto;
use std::env;
use std::fs;
use std::time::Instant;
use stwo_prover::core::{fields::m31::M31, vcs::blake2_merkle::Blake2sMerkleChannel};
use utils::{
    bench::{benchmark, Metrics},
    metadata::{FIBONACCI_INPUTS, SHA2_INPUTS},
    sha2_input,
};

/// Reference implementation of the Fibonacci function.
pub fn fib(n: u32) -> u32 {
    let mut a: M31 = M31(0);
    let mut b: M31 = M31(1);
    for _ in 1..n {
        let temp = a;
        a = b;
        b += temp;
    }
    b.0
}

fn bench_cairo_m_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    // Compile the program
    let source_path = "test_data/fibonacci_loop.cm".to_string();
    let source_text = fs::read_to_string(&source_path).expect("Failed to read fibonacci.cm");
    let options = CompilerOptions {
        verbose: false,
        optimization_level: Default::default(),
    };
    let output =
        compile_cairo(source_text, source_path, options).expect("Failed to compile fibonacci.cm");
    let compiled_program = (*output.program).clone();

    // Program Execution - Trace Generation
    let entrypoint_name = "fibonacci_loop".to_string();
    let runner_inputs = vec![InputValue::Number(n as i64)];

    let start = Instant::now();
    let runner_output = run_cairo_program(
        &compiled_program,
        entrypoint_name.as_str(),
        &runner_inputs,
        Default::default(),
    )
    .expect("failed to run cairo program");
    metrics.exec_duration = start.elapsed();

    // Return values
    let return_values: Vec<u32> = runner_output
        .return_values
        .iter()
        .map(|value| match value {
            cairo_m_common::CairoMValue::Felt(m31) => m31.0,
            cairo_m_common::CairoMValue::U32(u) => *u,
            _ => panic!("Unexpected return value type"),
        })
        .collect();
    assert_eq!(fib(n), return_values[0]);

    // Proof Generation
    let segment = runner_output.vm.segments.into_iter().next().unwrap();
    let mut prover_input = import_from_runner_output(segment, runner_output.public_address_ranges)
        .expect("failed to import from runner output");

    let pcs_config = REGULAR_96_BITS;

    let start = Instant::now();
    let proof = prove_cairo_m::<Blake2sMerkleChannel>(&mut prover_input, Some(pcs_config))
        .expect("failed to generate proof");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // verify proof
    let start = Instant::now();
    verify_cairo_m::<Blake2sMerkleChannel>(proof, Some(pcs_config))
        .expect("failed to verify proof");
    metrics.verify_duration = start.elapsed();

    metrics
}

/// Prepares a message for the Cairo-M SHA256 function by padding it and
/// converting it to u32 words.
fn prepare_sha256_input(msg: &[u8]) -> Vec<u32> {
    // Perform standard SHA-256 padding
    let mut padded_bytes = msg.to_vec();
    padded_bytes.push(0x80);

    // Pad to 56 bytes (448 bits) within the last chunk
    while padded_bytes.len() % 64 != 56 {
        padded_bytes.push(0x00);
    }

    // Append message length as 64-bit big-endian
    let bit_len = (msg.len() as u64) * 8;
    padded_bytes.extend_from_slice(&bit_len.to_be_bytes());

    // Convert bytes to u32 words (big-endian)
    padded_bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().expect("Chunk size mismatch")))
        .collect()
}

fn bench_cairo_m_sha256(num_bytes: u32) -> Metrics {
    let mut metrics = Metrics::new(num_bytes as usize);

    // Compile the program
    let source_path = "test_data/sha256.cm".to_string();
    let source_text = fs::read_to_string(&source_path).expect("Failed to read sha256.cm");
    let options = CompilerOptions {
        verbose: false,
        optimization_level: Default::default(),
    };
    let output =
        compile_cairo(source_text, source_path, options).expect("Failed to compile sha256.cm");
    let compiled_program = (*output.program).clone();

    // Generate input using sha2_input
    let input_bytes = sha2_input(num_bytes as usize);

    // Prepare the input with proper SHA-256 padding
    let padded_words = prepare_sha256_input(&input_bytes);

    // Select appropriate entry point based on input size
    let (entrypoint_name, runner_inputs) = match num_bytes {
        32 => {
            // Use sha256_32 entry point
            let input_values: Vec<InputValue> = padded_words[..16]
                .iter()
                .map(|&word| InputValue::Number(word as i64))
                .collect();
            ("sha256_32", vec![InputValue::List(input_values)])
        }
        256 => {
            // Use sha256_256 entry point
            let input_values: Vec<InputValue> = padded_words[..80]
                .iter()
                .map(|&word| InputValue::Number(word as i64))
                .collect();
            ("sha256_256", vec![InputValue::List(input_values)])
        }
        512 => {
            // Use sha256_512 entry point
            let input_values: Vec<InputValue> = padded_words[..144]
                .iter()
                .map(|&word| InputValue::Number(word as i64))
                .collect();
            ("sha256_512", vec![InputValue::List(input_values)])
        }
        1024 => {
            // Use sha256_1024 entry point
            let input_values: Vec<InputValue> = padded_words[..272]
                .iter()
                .map(|&word| InputValue::Number(word as i64))
                .collect();
            ("sha256_1024", vec![InputValue::List(input_values)])
        }
        2048 => {
            // Use sha256_2048 entry point
            let input_values: Vec<InputValue> = padded_words[..528]
                .iter()
                .map(|&word| InputValue::Number(word as i64))
                .collect();
            ("sha256_2048", vec![InputValue::List(input_values)])
        }
        _ => {
            eprintln!(
                "Error: Unsupported input size {}. Supported sizes are: 32, 256, 512, 1024, 2048",
                num_bytes
            );
            panic!("Unsupported SHA-256 input size");
        }
    };

    let start = Instant::now();
    let runner_output = run_cairo_program(
        &compiled_program,
        entrypoint_name,
        &runner_inputs,
        Default::default(),
    )
    .expect("failed to run cairo program");
    metrics.exec_duration = start.elapsed();

    // Debug: print return values
    eprintln!(
        "SHA-256 returned {} values",
        runner_output.return_values.len()
    );

    // SHA256 returns an array as a single return value
    let cairo_sha256: Vec<u32> = if runner_output.return_values.len() == 1 {
        match &runner_output.return_values[0] {
            cairo_m_common::CairoMValue::Array(arr) => arr
                .iter()
                .map(|v| match v {
                    cairo_m_common::CairoMValue::U32(u) => *u,
                    _ => panic!("Expected U32 value in SHA256 array output"),
                })
                .collect(),
            _ => panic!("Expected SHA256 to return an array of 8 u32 values"),
        }
    } else {
        panic!("Expected SHA256 to return exactly one value (an array)");
    };

    // Compute expected SHA256 using Rust's sha2 crate
    let mut hasher = Sha256::new();
    hasher.update(&input_bytes);
    let rust_result = hasher.finalize();

    // Convert Rust result to u32 array for comparison
    let rust_sha256: Vec<u32> = rust_result
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect();

    // Compare the results
    let matches = cairo_sha256 == rust_sha256;
    if !matches {
        eprintln!("WARNING: Cairo-M SHA256 output does not match Rust sha2 implementation!");
    }

    // Proof Generation
    let segment = runner_output.vm.segments.into_iter().next().unwrap();

    let mut prover_input = import_from_runner_output(segment, runner_output.public_address_ranges)
        .expect("Failed to import runner output for proof generation");

    let pcs_config = REGULAR_96_BITS;

    let start = Instant::now();
    let proof = prove_cairo_m::<Blake2sMerkleChannel>(&mut prover_input, Some(pcs_config))
        .expect("failed to generate proof");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // verify proof
    let start = Instant::now();
    verify_cairo_m::<Blake2sMerkleChannel>(proof, Some(pcs_config))
        .expect("failed to verify proof");
    metrics.verify_duration = start.elapsed();

    metrics
}

/// Runs a compiled Cairo program and generate a proof of execution.
///
/// ## Errors
///
/// Returns a `Error` if JSON parsing, VM execution, or proof generation fails.
fn main() {
    dotenv::dotenv().ok();

    let args: Vec<String> = env::args().collect();
    let benchmark_type = args.get(1).map(|s| s.as_str()).unwrap_or("fib");

    match benchmark_type {
        "fib" => {
            benchmark(
                bench_cairo_m_fib,
                &FIBONACCI_INPUTS,
                "../.outputs/benchmark/fib_cairo-m.csv",
            );
        }
        "sha256" => {
            // Convert usize inputs to u32 for consistency with the function signature
            let sha256_inputs: Vec<u32> = SHA2_INPUTS.iter().map(|&x| x as u32).collect();
            benchmark(
                bench_cairo_m_sha256,
                &sha256_inputs,
                "../.outputs/benchmark/sha2_cairo-m.csv",
            );
        }
        _ => {
            eprintln!(
                "Unknown benchmark type: {}. Use 'fib' or 'sha256'",
                benchmark_type
            );
            std::process::exit(1);
        }
    }
}
