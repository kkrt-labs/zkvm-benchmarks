use std::{env, fs::File, path::PathBuf, process::Command, time::Duration};
use zk_engine::{
    nova::{
        provider::{ipa_pc, Bn256EngineIPA},
        spartan,
        traits::Dual,
    },
    utils::logging::init_logger,
    wasm_ctx::{WASMArgsBuilder, WASMCtx, ZKWASMCtx},
    wasm_snark::{StepSize, WasmSNARK},
};

use utils::{benchmark, size};
type BenchResult = (Duration, usize, usize);

// Curve Cycle to prove/verify on
pub type E = Bn256EngineIPA;
pub type EE1 = ipa_pc::EvaluationEngine<E>;
pub type EE2 = ipa_pc::EvaluationEngine<Dual<E>>;
pub type S1 = spartan::batched::BatchedRelaxedR1CSSNARK<E, EE1>;
pub type S2 = spartan::batched::BatchedRelaxedR1CSSNARK<Dual<E>, EE2>;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "zkwasm-cli")]
#[command(about = "Example CLI to prove and verify WASM execution", long_about = None)]
struct Cli {
    #[arg(short, long)]
    guest: String,

    #[arg(short, long)]
    wat: Option<String>,

    #[arg(short, long, num_args = 0..)]
    benchmark_args: Vec<String>,

    #[arg(short = 's', long, default_value = "10")]
    execution_step_size: usize,

    #[arg(short = 's', long, default_value = "10")]
    memory_step_size: Option<usize>,

    #[arg(short, long)]
    compress: bool,
}

fn build_guest(package_name: &str) {
    println!("Current directory: {:?}", env::current_dir().unwrap());

    let output = Command::new("cargo")
        .env("MODEL", "e5small")
        .args(&[
            "build",
            "--release",
            "--target",
            "wasm32-unknown-unknown",
            "--package",
            package_name,
        ])
        .output()
        .expect("Failed to build WASM package");

    if !output.status.success() {
        panic!(
            "Building WASM package failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("WASM build completed.");

    let output_file =
        File::create(format!("wats/{}.wat", package_name)).expect("Failed to create output file");

    let output = Command::new("wasm2wat")
        .arg(format!(
            "target/wasm32-unknown-unknown/release/{}.wasm",
            package_name
        ))
        .stdout(output_file.try_clone().unwrap()) // Redirect standard output to the file
        .output() // Captures both stdout and stderr
        .expect("Failed to run wasm2wat");

    if !output.status.success() {
        panic!(
            "Candid extraction failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("Generated WAT file.");
}

fn main() {
    let cli = Cli::parse();

    init_logger();

    if cli.wat.is_none() {
        build_guest(&cli.guest);
    }

    benchmark(
        generate(cli.clone()),
        &cli.benchmark_args,
        &format!("../benchmark_outputs/{}_novanet.csv", cli.guest),
    );
}

fn generate(cli: Cli) -> impl Fn(String) -> BenchResult {
    move |n: String| {
        let func_args = vec![n];

        let mut step_size = StepSize::new(cli.execution_step_size);

        if let Some(ms) = cli.memory_step_size {
            step_size = step_size.set_memory_step_size(ms);
        }

        // Produce setup material
        let pp = WasmSNARK::<E, S1, S2>::setup(step_size);

        let wat_path = if let Some(wat_path) = cli.wat.clone() {
            wat_path
        } else {
            format!("wats/{}.wat", cli.guest)
        };

        println!("wat_path: {wat_path}");

        // Specify arguments to the WASM and use it to build a `WASMCtx`
        let wasm_args = WASMArgsBuilder::default()
            .file_path(PathBuf::from(wat_path))
            .unwrap()
            .invoke(&cli.guest)
            .func_args(func_args)
            .build();
        let wasm_ctx = WASMCtx::new(wasm_args);

        // Prove wasm execution
        let start = std::time::Instant::now();
        let (mut snark, instance) =
            WasmSNARK::<E, S1, S2>::prove(&pp, &wasm_ctx, step_size).expect("Failed in prove");

        // Compress the proof
        if cli.compress {
            snark = snark.compress(&pp, &instance).expect("Failed in compress");
        }

        // Verify the proof
        snark.verify(&pp, &instance).expect("Failed in verify");

        let end = std::time::Instant::now();
        let duration = end.duration_since(start);

        // Get execution trace length
        let (execution_trace, _, _) = wasm_ctx
            .execution_trace()
            .expect("Failed in execution_trace");

        println!("Success!");

        (duration, size(&snark), execution_trace.len())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_fib_without_memory_accessing() {
//         let cli = Cli {
//             guest: String::from("fib"),
//             wat: Some(String::from("../fibonacci/fib.wat")),
//             benchmark_args: vec![String::from("16"), String::from("17")],
//             execution_step_size: 10,
//             memory_step_size: None,
//             compress: true,
//         };

//         benchmark(
//             generate(cli.clone()),
//             &cli.benchmark_args,
//             &format!("../../benchmark_outputs/test_{}_novanet_{}_compressing.csv", cli.guest, if cli.compress {"with"} else {"without"}),
//             &format!("{}_arg", cli.guest),
//         );
//     }
// }
