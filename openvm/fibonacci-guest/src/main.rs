#![cfg_attr(not(feature = "std"), no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use openvm::io::{read, reveal_u32};

openvm::entry!(main);

pub fn main() {
    let n: u32 = read();
    let result = guests::fib::fib(n);
    reveal_u32(result, 0);
}
