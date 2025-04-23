#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use openvm::io::{read, reveal};

openvm::entry!(main);

pub fn main() {
    let n: u32 = read();
    let result = guests::fib::fib(n);
    reveal(result as u32, 0);
    reveal((result >> 32) as u32, 1);
}
