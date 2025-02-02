use powdr_riscv_runtime;
use powdr_riscv_runtime::commit;
use powdr_riscv_runtime::io::{read, write};

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

fn main() {
    // Read input from stdin.
    let n: u32 = read(0);
    let r = fib(n);
    // Write result to stdout.
    write(1, r);
    // Commit the result as a public.
    commit::commit(r.try_into().unwrap());
}
