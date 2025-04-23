#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use guests::fib;

#[nexus_rt::main]
fn main(n: u32) -> u128 {
    let result = fib::fib(n);
    result
}
