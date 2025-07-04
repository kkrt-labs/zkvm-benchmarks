use noir_fib::fib;

pub fn main(n: u32) -> pub Field {
    fib::<100>(n)
}

#[test]
fn test_main() {
    let result = main(100);
    assert(result == 0x1333db76a7c594bfc3);
}
