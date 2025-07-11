use std::{path::Path, time::Instant};

use utils::{
    bench::{benchmark, Metrics},
    metadata::FIBONACCI_INPUTS,
};

use cairo_air::verifier::verify_cairo;
use cairo_air::PreProcessedTraceVariant;
use cairo_prove::prove::{prove, prover_input_from_runner};
use cairo_vm::{
    cairo_run::{cairo_run_program, CairoRunConfig},
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    types::{layout_name::LayoutName, program::Program},
};
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

    let path = format!("test_data/fibonacci_{}.json", n);
    let entrypoint = "main";
    let program = Program::from_file(Path::new(&path), Some(entrypoint))
        .expect("Failed to read Cairo Zero program");

    let config = CairoRunConfig {
        entrypoint: "main",
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
    };

    let mut hint_processor = BuiltinHintProcessor::new_empty();

    let pcs_config = REGULAR_96_BITS;

    // Execute.
    let start_time = Instant::now();
    let runner = cairo_run_program(&program, &config, &mut hint_processor)
        .expect("Failed to run Cairo Zero program");

    metrics.exec_duration = start_time.elapsed();

    // Prove.
    let start_time = Instant::now();
    let prover_input = prover_input_from_runner(&runner);
    let proof = prove(prover_input, pcs_config);
    metrics.proof_duration = start_time.elapsed();
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // Verify.
    let start_time = Instant::now();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    let result = verify_cairo::<Blake2sMerkleChannel>(proof, pcs_config, preprocessed_trace);
    assert!(result.is_ok());
    metrics.verify_duration = start_time.elapsed();

    metrics
}

/// Runs a compiled Cairo Zero program and generate a proof of execution.
fn main() {
    dotenv::dotenv().ok();
    benchmark(
        bench_cairo_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_cairo-zero.csv",
    );
}
