use nexus_sdk::{
    ByGuestCompilation, Local, Prover,
    compile::{Compile, Compiler, cargo::CargoPackager},
    stwo::seq::Stwo,
};
use utils::profile::profile_func;

const PACKAGE: &str = "fibonacci-guest";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 100000;
    println!("Compiling guest program...");
    let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
    let prover: Stwo<Local> =
        Stwo::compile(&mut prover_compiler).expect("failed to compile guest program");

    println!("Proving execution of vm...");
    profile_func(
        || {
            let (_view, _proof) = prover
                .prove_with_input::<u32, ()>(&n, &())
                .expect("failed to prove program");
        },
        "../profile_outputs/profile_nexus.pb",
    )?;

    Ok(())
}
