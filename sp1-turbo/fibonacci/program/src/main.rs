//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use fibonacci_lib::{fibonacci, PublicValuesStruct};

pub fn main() {
    let n = sp1_zkvm::io::read::<u32>();

    let (a, b) = fibonacci(n);

    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });

    sp1_zkvm::io::commit_slice(&bytes);
}
