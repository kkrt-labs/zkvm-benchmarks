#[executable]
fn main(n: felt252) -> felt252 {
    let result = fib(n);
    result
}

pub fn fib(n_felt: felt252) -> felt252 {
    let mut a = 0;
    let mut b = 1;
    let n: u32 = n_felt.try_into().unwrap();
    for _ in 1..n {
        let temp = a;
        a = b;
        b += temp;
    }
    b.into()
}
