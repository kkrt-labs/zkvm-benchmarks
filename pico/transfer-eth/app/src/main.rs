#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};
use revm_utils::transfer_eth_n_times;

pub fn main() {
    let n: usize = read_as();

    let result = transfer_eth_n_times(n);
    assert!(result);

    commit(&result);
}
