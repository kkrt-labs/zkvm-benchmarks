use noir_fib::fib;

pub fn main() -> pub Field {
    fib::<100>()
}

#[test]
fn test_main() {
    let result = main();
    assert(result == 0x1333db76a7c594bfc3);
}
