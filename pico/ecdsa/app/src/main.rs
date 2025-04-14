#![no_main]

pico_sdk::entrypoint!(main);
use ecdsa_lib::{EcdsaData, ecdsa_verify};
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let input = read_as::<EcdsaData>();
    let result = ecdsa_verify(input.encoded_point, &input.message, input.signature);
    assert!(result, "Signature verification failed");
    commit(&result);
}
