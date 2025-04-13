#![no_main]

pico_sdk::entrypoint!(main);
use ethblock_lib::{EthblockData, trace_block};
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let num_txs = read_as::<usize>();
    let b = trace_block(num_txs);
    assert!(b);

    let result = EthblockData { is_ok: b };

    commit(&result);
}
