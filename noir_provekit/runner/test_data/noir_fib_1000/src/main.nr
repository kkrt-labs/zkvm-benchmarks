use noir_fib::fib;

pub fn main() -> pub Field {
    fib::<1000>()
}

#[test]
fn test_main() {
    let result = main();
    assert(result == 0x1caaf75c2b082b66faceefb6b87947ffb3da12fe58cd807f496b703e6c98f8b7);
}
