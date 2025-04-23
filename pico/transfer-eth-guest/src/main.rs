#![no_main]

pico_sdk::entrypoint!(main);
use guests::ethtransfer;
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let n: usize = read_as();
    let result = ethtransfer::ethtransfer(n);
    commit(&result);
}
