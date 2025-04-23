#![no_main]

pico_sdk::entrypoint!(main);
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let num_txs = read_as::<usize>();
    let is_ok = ethblock_utils::trace_ethblock(num_txs);
    assert!(is_ok);
    commit(&is_ok);
}
