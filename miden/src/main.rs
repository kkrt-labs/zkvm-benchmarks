use miden_processor::execute;
use miden_processor::math::Felt;
use miden_processor::ExecutionOptions;
use miden_vm::{
    assembly::DefaultSourceManager, prove, verify, Assembler, DefaultHost, ProgramInfo,
    ProvingOptions,
};
use miden_vm::{AdviceInputs, StackInputs, StackOutputs};
use std::fs;
use std::sync::Arc;
use std::time::Instant;
use utils::{
    bench::{benchmark, Metrics},
    metadata::FIBONACCI_INPUTS,
};

// Rust reference
fn fibonacci(n: u32) -> Felt {
    let mut a: Felt = Felt::new(0);
    let mut b: Felt = Felt::new(1);
    for _ in 1..n {
        let tmp = a;
        a = b;
        b += tmp;
    }
    b
}

fn bench_miden_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    // Compile the program
    let assembler = Assembler::default();
    let program_string = fs::read_to_string("src/fibonacci_repeat.masm")
        .expect("Failed to read fibonacci_repeat.masm")
        .replace("Z", &(n - 1).to_string());

    let program = assembler
        .assemble_program(program_string)
        .expect("Failed to assemble fibonacci_repeat.masm program");

    // Prepare inputs
    let stack_inputs =
        StackInputs::new(vec![Felt::from(1_u32)]).expect("Failed to create stack inputs");
    let advice_inputs = AdviceInputs::default();
    let source_manager = Arc::new(DefaultSourceManager::default());

    // Execute
    let execution_start = Instant::now();
    let trace = execute(
        &program,
        stack_inputs.clone(),
        advice_inputs.clone(),
        &mut DefaultHost::default(),
        ExecutionOptions::default(),
        source_manager,
    )
    .expect("Failed to execute Miden program");
    metrics.exec_duration = execution_start.elapsed();
    metrics.cycles = trace.get_trace_len() as u64;

    // Prove and execute (not possible to isolate the proof generation due to ExecutionProver being private)
    // An approximation of the proof duration is done by subtracting the execution duration from the total duration
    let execution_proving_start = Instant::now();
    let source_manager = Arc::new(DefaultSourceManager::default());
    let (outputs, proof) = prove(
        &program,
        stack_inputs.clone(),
        advice_inputs.clone(),
        &mut DefaultHost::default(),
        ProvingOptions::default(),
        source_manager,
    )
    .expect("Failed to prove Miden program execution");
    let execution_proving_end = execution_proving_start.elapsed();
    metrics.proof_duration = execution_proving_end - metrics.exec_duration;
    metrics.proof_bytes = proof.to_bytes().len();

    let expected_output = fibonacci(n);
    assert_eq!(outputs.get_stack_item(0).unwrap(), expected_output);

    // Verify
    let verification_start = Instant::now();
    verify(
        ProgramInfo::new(program.hash(), program.kernel().clone()),
        stack_inputs.clone(),
        StackOutputs::new(vec![expected_output, fibonacci(n - 1)]).unwrap(),
        proof,
    )
    .expect("Failed to verify Miden proof");
    metrics.verify_duration = verification_start.elapsed();

    metrics
}

fn main() {
    dotenv::dotenv().ok();
    benchmark(
        bench_miden_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_miden.csv",
    );
}
