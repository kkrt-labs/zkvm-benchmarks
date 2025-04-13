#![no_main]

pico_sdk::entrypoint!(main);
use fibonacci_lib::{fibonacci, FibonacciData};
use pico_sdk::io::{commit, read_as};

pub fn main() {
    // Read inputs `n` from the environment
    let n: u32 = read_as();

    // Compute Fibonacci values starting from `a` and `b`
    let (a_result, b_result) = fibonacci(n);

    // Commit the assembled Fibonacci data as the public values in the Pico proof.
    // This allows the values to be verified by others.
    let result = FibonacciData {
        n,
        a: a_result,
        b: b_result,
    };

    commit(&result);
}
