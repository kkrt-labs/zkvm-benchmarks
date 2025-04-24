use utils::profile::profile_func;
use zkm_script::init_logger;

use zkm_build::include_elf;
use zkm_sdk::{ProverClient, ZKMStdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 10;
    init_logger();

    let mut stdin = ZKMStdin::new();
    stdin.write(&n);

    const FIBONACCI_ELF: &[u8] = include_elf!("fibonacci");
    let client = ProverClient::cpu();
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    println!("benchmark start");
    profile_func(
        || {
            let _proof = client.prove(&pk, stdin.clone()).run().unwrap();
        },
        "../profile_outputs/profile_zkm.pb",
    )?;

    Ok(())
}
