use noir_fib::fib;

pub fn main(n: u32) -> pub Field {
    fib::<10>(n)
}

#[test]
fn test_main() {
    let result = main(10);
    assert(result == 0x37);
}
