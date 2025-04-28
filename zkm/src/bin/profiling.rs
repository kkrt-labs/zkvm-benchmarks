use utils::{ecdsa_input, profile::profile_func};
use zkm_script::init_logger;

use zkm_build::include_elf;
use zkm_sdk::{ProverClient, ZKMStdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = ecdsa_input();
    init_logger();

    let mut stdin = ZKMStdin::new();
    stdin.write(&input);

    const ELF: &[u8] = include_elf!("ecdsa-bench");
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(ELF);

    println!("benchmark start");
    profile_func(
        || {
            let _proof = client.prove(&pk, stdin.clone()).run().unwrap();
        },
        "../.outputs/profiling/profile_zkm.pb",
    )?;

    Ok(())
}
