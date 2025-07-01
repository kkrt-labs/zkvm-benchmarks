use noir_fib::fib;

pub fn main() -> pub Field {
    fib::<10>()
}

#[test]
fn test_main() {
    let result = main();
    assert(result == 0x37);
}
