pub fn fib<let N: u32>() -> Field {
    let mut a: Field = 0;
    let mut b: Field = 1;

    for _ in 1..N {
        let temp = a;
        a = b;
        b = temp + b;
    }
    b
}

#[test]
fn test_fib() {
    let result = fib::<10>();
    assert(result == 0x37);
}
