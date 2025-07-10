#[executable]
fn main(n: felt252) -> felt252 {
    let result = fib(n);
    result
}

pub fn fib(n: felt252) -> felt252 {
    let mut a = 0;
    let mut b = 1;
	let i: u32 = n.try_into().unwrap();
    for _ in 1..i {
        let temp = a;
        a = b;
        b += temp;
    }
    b.into()
}
