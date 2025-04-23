#![no_main]

pico_sdk::entrypoint!(main);
use guests::fib;
use pico_sdk::io::{commit, read_as};

pub fn main() {
    let n: u32 = read_as();
    let result = fib::fib(n);
    commit(&result);
}
