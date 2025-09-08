use cairo_air::verifier::verify_cairo;
use cairo_air::CairoProof;
use cairo_air::PreProcessedTraceVariant;
use cairo_lang_casm::hints::Hint;
use cairo_lang_executable::executable::{EntryPointKind, Executable};
use cairo_lang_runner::Arg;
use cairo_lang_runner::CairoHintProcessor;
use cairo_lang_runner::build_hints_dict;
use cairo_vm::cairo_run::{cairo_run_program, CairoRunConfig};
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use cairo_vm::Felt252;
use log::info;
use sonic_rs;
use std::collections::HashMap;
use std::env;
use stwo_cairo_adapter::builtins::MemorySegmentAddresses;
use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig, MemoryEntry};
use stwo_cairo_adapter::vm_import::{adapt_to_stwo_input, RelocatedTraceEntry};
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_prover::stwo_prover::core::fri::FriConfig;
use stwo_cairo_prover::stwo_prover::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use utils::{
    bench::{benchmark, Metrics},
    metadata::{FIBONACCI_INPUTS, SHA2_INPUTS},
    sha2_input,
};

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

/// Executes a Cairo program and returns a `CairoRunner` that can be used to generate artifacts for
/// the prover.
pub fn execute(executable: Executable, args: Vec<Arg>) -> CairoRunner {
    let (program, string_to_hint) = program_and_hints_from_executable(&executable);

    let mut hint_processor = CairoHintProcessor {
        runner: None,
        user_args: vec![vec![Arg::Array(args)]],
        string_to_hint,
        starknet_state: Default::default(),
        run_resources: Default::default(),
        syscalls_used_resources: Default::default(),
        no_temporary_segments: false,
        markers: Default::default(),
        panic_traceback: Default::default(),
    };

    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_mem: true,
        layout: LayoutName::all_cairo_stwo,
        secure_run: None,
        allow_missing_builtins: None,
        dynamic_layout_params: None,
        disable_trace_padding: true,
        proof_mode: true,
        ..Default::default()
    };

    info!("Executing program...");
    let runner = cairo_run_program(&program, &cairo_run_config, &mut hint_processor)
        .expect("Failed to execute program");
    info!("Program executed successfully.");
    runner
}

fn program_and_hints_from_executable(executable: &Executable) -> (Program, HashMap<String, Hint>) {
    let data: Vec<MaybeRelocatable> = executable
        .program
        .bytecode
        .iter()
        .map(Felt252::from)
        .map(MaybeRelocatable::from)
        .collect();
    let (hints, string_to_hint) = build_hints_dict(&executable.program.hints);
    let entrypoint = executable
        .entrypoints
        .iter()
        .find(|e| matches!(e.kind, EntryPointKind::Standalone))
        .expect("Failed to find entrypoint");
    let program = Program::new_for_proof(
        entrypoint.builtins.clone(),
        data,
        entrypoint.offset,
        entrypoint.offset + 4,
        hints,
        Default::default(),
        Default::default(),
        vec![],
        None,
    )
    .unwrap();
    (program, string_to_hint)
}

///
/// Replacement of the functions from the `cairo-prove` crate that are conflicting with
/// the revision of `stwo` to use.
///

// Deduces the preprocessed trace variant needed for the specific execution, and proves.
pub fn prove(input: ProverInput, pcs_config: PcsConfig) -> CairoProof<Blake2sMerkleHasher> {
    // Currently there are two variants of the preprocessed trace:
    // - Canonical: Pedersen is included in the program.
    // - CanonicalWithoutPedersen: Pedersen is not included in the program.
    // We deduce the variant based on weather the pedersen builtin is included in the program.
    let preprocessed_trace = match input.public_segment_context[1] {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };
    prove_inner(input, preprocessed_trace, pcs_config)
}

fn prove_inner(
    input: ProverInput,
    preprocessed_trace: PreProcessedTraceVariant,
    pcs_config: PcsConfig,
) -> CairoProof<Blake2sMerkleHasher> {
    stwo_cairo_prover::prover::prove_cairo::<Blake2sMerkleChannel>(
        input,
        pcs_config,
        preprocessed_trace,
    )
    .unwrap()
}

pub fn prover_input_from_runner(runner: &CairoRunner) -> ProverInput {
    let public_input = runner.get_air_public_input().unwrap();
    let addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect::<Vec<_>>();
    let segments = public_input
        .memory_segments
        .iter()
        .map(|(&k, v)| {
            (
                k,
                MemorySegmentAddresses {
                    begin_addr: v.begin_addr,
                    stop_ptr: v.stop_ptr,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let trace = runner
        .relocated_trace
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| RelocatedTraceEntry {
            ap: x.ap,
            fp: x.fp,
            pc: x.pc,
        })
        .collect::<Vec<_>>();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.as_ref().map(|value| MemoryEntry {
                address: i as u64,
                value: unsafe { std::mem::transmute::<[u8; 32], [u32; 8]>(value.to_bytes_le()) },
            })
        });
    let mem = MemoryBuilder::from_iter(MemoryConfig::default(), mem);
    let main_args = runner
        .get_program()
        .iter_builtins()
        .copied()
        .collect::<Vec<_>>();

    let main_args_slice: &[BuiltinName] = &main_args;
    let public_segment_context = PublicSegmentContext::new(main_args_slice);

    let input =
        adapt_to_stwo_input(&trace, mem, addresses, &segments, public_segment_context).unwrap();
    input
}

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

    // Generate the input bytes using sha2_input
    let input_bytes = sha2_input(n as usize);

    // Convert bytes to felt252 arguments - pass as a single array argument
    // Cairo arrays need their length as the first element
    let mut args = vec![Arg::Value(Felt252::from(input_bytes.len()))];
    args.extend(
        input_bytes
            .into_iter()
            .map(|b| Arg::Value(Felt252::from(b))),
    );

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
