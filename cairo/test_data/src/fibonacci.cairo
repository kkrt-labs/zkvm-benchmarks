use core::felt252;

#[executable]
fn main(n: u128) -> felt252 {
    let result = fib(n);
    result
}

fn fib(n: u128) -> felt252 {
    let mut i: u128 = 0;
    let mut a = 0;
    let mut b = 1;
    let threshold = n - 2;
    loop {
        if i > threshold {
            break;
        }
        let tmp = a + b;
        a = b;
        b = tmp;
        i = i + 1;
    }
    return b;
}
