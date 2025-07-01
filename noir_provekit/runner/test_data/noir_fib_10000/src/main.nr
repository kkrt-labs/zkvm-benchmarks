use noir_fib::fib;

pub fn main() -> pub Field {
    fib::<10000>()
}

#[test]
fn test_main() {
    let result = main();
    assert(result == 0x29674c1a7d77ff68b1e666292977a841969556b9385f671a06b19b9b2e03c6e2);
}
