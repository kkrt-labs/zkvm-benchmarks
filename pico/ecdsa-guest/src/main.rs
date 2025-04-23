#![no_main]

pico_sdk::entrypoint!(main);
use guests::ecdsa::{ecdsa_verify, EcdsaVerifyInput};
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let input = read_as::<EcdsaVerifyInput>();
    let result = ecdsa_verify(input);
    assert!(result, "Signature verification failed");
    commit(&result);
}
