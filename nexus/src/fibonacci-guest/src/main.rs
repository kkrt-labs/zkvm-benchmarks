#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use nexus_rt::{read_private_input, write_output};

#[nexus_rt::main]
fn main() {
    let n = read_private_input::<u32>().unwrap();
    let z = fib(n);
    write_output::<u128>(&z)
}

fn fib(n: u32) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    b
}
