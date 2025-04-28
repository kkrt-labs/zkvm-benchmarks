use pico_sdk::client::DefaultProverClient;
use utils::{ecdsa_input, load_elf, profile::profile_func};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let elf = load_elf("./ecdsa-guest/elf/riscv32im-pico-zkvm-elf");
    let client = DefaultProverClient::new(&elf);
    let stdin_builder = client.get_stdin_builder();

    let input = ecdsa_input();
    stdin_builder.borrow_mut().write(&input);

    profile_func(
        || {
            client.prove_fast().expect("Failed to generate proof");
        },
        "../.outputs/profiling/profile_pico.pb",
    )?;

    Ok(())
}
