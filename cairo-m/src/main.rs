use cairo_m_compiler::{compile_cairo, CompilerOptions};
use cairo_m_prover::{
    adapter::import_from_runner_output, prover::prove_cairo_m, prover_config::REGULAR_96_BITS,
    verifier::verify_cairo_m,
};
use cairo_m_runner::run_cairo_program;
use std::fs;
use std::time::Instant;
use stwo_prover::core::{fields::m31::M31, vcs::blake2_merkle::Blake2sMerkleChannel};
use utils::{
    bench::{benchmark, Metrics},
    metadata::FIBONACCI_INPUTS,
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

fn bench_cairo_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    // Compile the program
    let source_path = "test_data/fibonacci_loop.cm".to_string();
    let source_text = fs::read_to_string(&source_path).expect("Failed to read fibonacci.cm");
    let options = CompilerOptions { verbose: false };
    let output =
        compile_cairo(source_text, source_path, options).expect("Failed to compile fibonacci.cm");
    let compiled_program = (*output.program).clone();

    // Program Execution - Trace Generation
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
    benchmark(
        bench_cairo_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_cairo-m.csv",
    );
}
