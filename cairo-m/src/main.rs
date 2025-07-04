use cairo_m_common::Program;
use cairo_m_prover::{
    adapter::import_from_runner_output, prover::prove_cairo_m, verifier::verify_cairo_m,
};
use cairo_m_runner::run_cairo_program;
use std::time::Instant;
use stwo_prover::core::{fields::m31::M31, vcs::blake2_merkle::Blake2sMerkleChannel};
use utils::{
    bench::{benchmark, Metrics},
    metadata::FIBONACCI_INPUTS,
};

/// Represents the possible errors that can occur in the VM.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JSON parsing error: {0}")]
    Json(String),
    #[error("VM Error: {0}")]
    Vm(String),
    #[error("Adapter error: {0}")]
    Adapter(String),
    #[error("Proof generation error: {0}")]
    Proof(String),
}

/// The result and metrics of a successful program execution and proof generation.
///
/// # Fields
///
/// * `return_values` - The return values of the program
/// * `num_steps` - The number of execution steps
/// * `overall_duration` - The total time for execution and proof generation, in seconds
/// * `execution_duration` - The time for execution, in seconds
/// * `proof_duration` - The time for proof generation, in seconds
/// * `overall_frequency` - The frequency of the execution and proof generation, in Hz
/// * `execution_frequency` - The frequency of the execution, in Hz
/// * `proof_frequency` - The frequency of the proof generation, in Hz
/// * `proof_size` - The size of the proof, in bytes
/// * `proof` - The proof of the program, serialized as a JSON string
#[derive(Debug)]
pub struct RunProofResult {
    pub return_values: Vec<u32>,
    pub num_steps: u32,
    pub overall_duration: f64,
    pub execution_duration: f64,
    pub proof_duration: f64,
    pub overall_frequency: f64,
    pub execution_frequency: f64,
    pub proof_frequency: f64,
    pub proof_size: u32,
    pub proof: String,
}

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

fn bench_cairo_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    let program_json_str = std::fs::read_to_string("test_data/fibonacci_loop.json")
        .expect("failed to read fibonacci_loop.json");

    // Program Execution - Trace Generation

    let compiled_program: Program =
        sonic_rs::from_str(&program_json_str).expect("failed to parse program json");

    let entrypoint_name = "fibonacci_loop".to_string();
    let entrypoint = compiled_program
        .get_entrypoint(&entrypoint_name)
        .unwrap_or_else(|| panic!("Entrypoint {} not found", entrypoint_name));

    let runner_inputs: Vec<M31> = [M31::from(n)]
        .iter()
        .take(entrypoint.args.len())
        .copied()
        .collect();

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
        .map(|value| value.0)
        .collect();
    assert_eq!(fib(n), return_values[0]);

    // Metrics Computation
    metrics.cycles = runner_output.vm.trace.len() as u64;

    // Proof Generation
    let mut prover_input =
        import_from_runner_output(runner_output).expect("failed to import from runner output");

    let start = Instant::now();
    let proof =
        prove_cairo_m::<Blake2sMerkleChannel>(&mut prover_input).expect("failed to generate proof");
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // verify proof
    let start = Instant::now();
    verify_cairo_m::<Blake2sMerkleChannel>(proof).expect("failed to verify proof");
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
    benchmark(
        bench_cairo_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_cairo-m.csv",
    );
}
