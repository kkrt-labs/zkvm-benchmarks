use acvm::FieldElement;
use bn254_blackbox_solver::Bn254BlackBoxSolver;
use nargo::{foreign_calls::DefaultForeignCallBuilder, ops::execute_program};
use noir_r1cs::NoirProof;
use noir_r1cs::NoirProofScheme;
use noirc_abi::{
    input_parser::{Format, InputValue},
    InputMap, MAIN_RETURN_NAME,
};
use noirc_artifacts::program::ProgramArtifact;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;
use utils::{
    bench::{benchmark, Metrics},
    metadata::FIBONACCI_INPUTS,
};

/// Errors to wrap ProveKit errors.
#[derive(Debug, thiserror::Error)]
pub enum NoirProverError {
    /// Error when instantiating a Noir prover with a given compiled Noir circuit.
    #[error("Failed to create prover: {0}")]
    CreationError(String),
    /// Something went wrong during the ProveKit proof generation.
    #[error("Failed to generate proof: {0}")]
    ProofGenerationError(String),
    /// The verification of the given Noir proof failed.
    #[error("Failed to verify proof: {0}")]
    VerificationError(String),
}

/// The proof and metrics of a successful Noir proof generation.
///
/// # Fields
///
/// * `execution_duration` - The duration of the execution of the program.
/// * `proof_duration` - The duration of the proof generation.
/// * `proof` - The proof generated.
#[derive(Debug)]
pub struct NoirProofResult {
    pub execution_duration: Duration,
    pub proof_duration: Duration,
    pub proof: NoirProof,
}

/// Object to generate a proof of a Noir circuit with ProveKit.
#[derive(Serialize, Deserialize)]
pub struct NoirProver {
    proof_scheme: NoirProofScheme,
    program: ProgramArtifact,
}

impl NoirProver {
    /// Generate the proof scheme of the given Noir circuit.
    /// * `circuit_json_str` - The compiled Noir circuit JSON as a string.
    pub fn from_circuit(circuit_json_str: &String) -> Result<Self, NoirProverError> {
        let program: ProgramArtifact = {
            sonic_rs::from_str(&circuit_json_str.to_string())
                .map_err(|e| NoirProverError::CreationError(e.to_string()))?
        };

        let proof_scheme = NoirProofScheme::from_program(program.clone())
            .map_err(|e| NoirProverError::CreationError(e.to_string()))?;
        Ok(Self {
            proof_scheme,
            program,
        })
    }

    /// Generates a proof of the loaded Noir circuit with detailed metrics.
    fn prove(&self, n: u32) -> Result<NoirProofResult, NoirProverError> {
        let exec_start = Instant::now();

        // Witness generation
        let input_json_str = format!(r#"{{"n": "0x{:X}"}}"#, n);
        let (input_map, _) = self.generate_witness_map(&input_json_str)?;
        let initial_witness = self.program.abi.encode(&input_map, None).map_err(|e| {
            NoirProverError::CreationError(format!("Failed to encode witness: {}", e))
        })?;
        let mut foreign_call_executor = DefaultForeignCallBuilder::default()
            .with_mocks(false)
            .build::<FieldElement>();
        let blackbox_solver = Bn254BlackBoxSolver(false);
        let mut witness_stack = execute_program(
            &self.program.bytecode,
            initial_witness,
            &blackbox_solver,
            &mut foreign_call_executor,
        )
        .map_err(|e| NoirProverError::CreationError(format!("Failed to execute program: {}", e)))?;
        let witness_map = witness_stack
            .pop()
            .ok_or_else(|| {
                NoirProverError::CreationError("No witness stack available".to_string())
            })?
            .witness;

        let (input_map, _) = self.program.abi.decode(&witness_map).map_err(|e| {
            NoirProverError::CreationError(format!("Failed to decode witness: {}", e))
        })?;
        let execution_duration = exec_start.elapsed();

        // Proof generation
        let proof_start = Instant::now();
        let proof = self
            .proof_scheme
            .prove(&input_map)
            .map_err(|e| NoirProverError::ProofGenerationError(e.to_string()))?;
        let proof_duration = proof_start.elapsed();
        Ok(NoirProofResult {
            execution_duration: execution_duration,
            proof_duration: proof_duration,
            proof: proof,
        })
    }

    /// Generate the ACIR witness map expected by the `ProveKit::prove` function from the input JSON
    /// string.
    /// * `input_json_str` - The circuit inputs in a JSON format as a string.
    fn generate_witness_map(
        &self,
        input_json_str: &str,
    ) -> Result<(InputMap, Option<InputValue>), NoirProverError> {
        let has_params = !self.program.abi.parameters.is_empty();
        let has_input = !input_json_str.is_empty();

        // If no parameters expected and no input provided, return empty map
        if !has_params && !has_input {
            return Ok((BTreeMap::new(), None));
        }

        // If parameters expected but no input provided, error
        if has_params && !has_input {
            return Err(NoirProverError::CreationError(String::from(
                "The ABI expects parameters but no input were provided.",
            )));
        }

        // Parse the input (handles both cases: params expected or not)
        let mut inputs = Format::Json
            .parse(input_json_str, &self.program.abi)
            .map_err(|e| NoirProverError::CreationError(e.to_string()))?;
        let return_value = inputs.remove(MAIN_RETURN_NAME);

        Ok((inputs, return_value))
    }
}

fn bench_noir_fib(n: u32) -> Result<Metrics, NoirProverError> {
    let mut metrics = Metrics::new(n as usize);

    let circuit_path_str = format!("runner/test_data/target/noir_fib_{}.json", n);
    let circuit_path = Path::new(&circuit_path_str);
    let circuit_json_str = std::fs::read_to_string(circuit_path).map_err(|e| {
        NoirProverError::CreationError(format!("Failed to read circuit file: {}", e))
    })?;

    // Create prover
    let prover = NoirProver::from_circuit(&circuit_json_str)?;

    // Generate proof
    let result = prover.prove(n)?;
    let proof_data = sonic_rs::to_string(&result.proof)
        .map_err(|e| NoirProverError::VerificationError(e.to_string()))?;
    metrics.proof_bytes = proof_data.len();
    metrics.exec_duration = result.execution_duration;
    metrics.proof_duration = result.proof_duration;

    // Verification
    let verify_start = Instant::now();
    prover
        .proof_scheme
        .verify(&result.proof)
        .map_err(|e| NoirProverError::VerificationError(e.to_string()))?;
    metrics.verify_duration = verify_start.elapsed();

    Ok(metrics)
}

fn main() {
    dotenv::dotenv().ok();

    let bench_fn = |n: u32| -> Metrics {
        match bench_noir_fib(n) {
            Ok(metrics) => metrics,
            Err(e) => {
                eprintln!("Benchmark failed for n={}: {}", n, e);
                std::process::exit(1);
            }
        }
    };

    benchmark(
        bench_fn,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_noir-provekit.csv",
    );
}
