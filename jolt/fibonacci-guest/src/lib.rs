#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use guests::fib;

#[jolt::provable]
fn fib(n: u32) -> u32 {
    fib::fib(n)
}
