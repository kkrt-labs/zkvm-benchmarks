#![no_main]

pico_sdk::entrypoint!(main);
use guests::sha2;
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let input = read_as::<Vec<u8>>();
    let result = sha2::sha2(&input);
    commit(&result);
}
