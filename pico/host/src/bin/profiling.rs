use pico_sdk::client::DefaultProverClient;
use utils::{load_elf, profile::profile_func};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 10;
    let elf = load_elf("fibonacci-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();
    stdin_builder.borrow_mut().write(&n);

    println!("n: {}", n);

    profile_func(
        || {
            client.prove_fast().expect("Failed to generate proof");
        },
        "../profile_outputs/profile_pico.pb",
    )?;

    Ok(())
}
