//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use guests::fib;

pub fn main() {
    let n = sp1_zkvm::io::read::<u32>();
    let result = fib::fib(n);
    sp1_zkvm::io::commit(&result);
}
