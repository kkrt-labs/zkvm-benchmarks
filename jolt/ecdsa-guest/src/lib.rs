#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use guests::ecdsa;

#[jolt::provable(
    stack_size = 1000_000,
    memory_size = 10000000,
    max_input_size = 10000000
)]
pub fn ecdsa_verify(input: ecdsa::EcdsaVerifyInput) -> bool {
    ecdsa::ecdsa_verify(input)
}
