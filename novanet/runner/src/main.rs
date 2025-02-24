use std::path::PathBuf;
use zk_engine::{
    nova::{
        provider::{ipa_pc, Bn256EngineIPA},
        spartan,
        traits::Dual,
    },
    {
        error::ZKWASMError,
        utils::logging::init_logger,
        wasm_ctx::{WASMArgsBuilder, WASMCtx},
        wasm_snark::{StepSize, WasmSNARK},
    },
};

// Curve Cycle to prove/verify on
pub type E = Bn256EngineIPA;
pub type EE1 = ipa_pc::EvaluationEngine<E>;
pub type EE2 = ipa_pc::EvaluationEngine<Dual<E>>;
pub type S1 = spartan::batched::BatchedRelaxedR1CSSNARK<E, EE1>;
pub type S2 = spartan::batched::BatchedRelaxedR1CSSNARK<Dual<E>, EE2>;

use clap::Parser;

/// コマンドライン引数を管理するための構造体
#[derive(Parser, Debug)]
#[command(name = "zkwasm-cli")]
#[command(about = "Example CLI to prove and verify WASM execution", long_about = None)]
struct Cli {
    /// 実行する .wat または .wasm のファイルパス
    #[arg(short, long, value_name = "FILE")]
    file_path: PathBuf,

    /// 呼び出したい関数名
    #[arg(short, long)]
    invoke: String,

    /// WASM関数に渡す引数（複数指定可）
    #[arg(short, long, num_args = 0..)]
    func_args: Vec<String>,

    /// Step size（WASM 実行を証明する際の1ステップあたりの最大命令数）
    #[arg(short = 's', long, default_value = "10")]
    step_size: usize,
}

fn main() -> Result<(), ZKWASMError> {

    let cli = Cli::parse();

    init_logger();

    // Specify step size.
    //
    // Here we choose `10` as the step size because the wasm execution of fib(16) is 253 opcodes.
    // meaning zkWASM will run for 26 steps (rounds up).
    let step_size = StepSize::new(cli.step_size);

    // Produce setup material
    let pp = WasmSNARK::<E, S1, S2>::setup(step_size);

    // Specify arguments to the WASM and use it to build a `WASMCtx`
    let wasm_args = WASMArgsBuilder::default()
        .file_path(PathBuf::from(cli.file_path))
        .unwrap()
        .invoke(&cli.invoke)
        .func_args(cli.func_args)
        .build();
    let wasm_ctx = WASMCtx::new(wasm_args);

    // Prove wasm execution of fib.wat::fib(16)
    let (snark, instance) = WasmSNARK::<E, S1, S2>::prove(&pp, &wasm_ctx, step_size)?;

    // Verify the proof
    snark.verify(&pp, &instance)?;

    println!("Success!");

    Ok(())
}
