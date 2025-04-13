#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};
use sha256_lib::{Sha256Data, hash_sha256};

pub fn main() {
    let input = read_as::<Vec<u8>>();

    let digest = hash_sha256(input.clone());

    // Commit the assembled Sha256 data as the public values in the Pico proof.
    // This allows the values to be verified by others.
    let result = Sha256Data {
        input: input.clone(),
        result: digest,
    };

    commit(&result);
}
