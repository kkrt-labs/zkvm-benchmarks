use noir_fib::fib;

pub fn main(n: u32) -> pub Field {
    fib::<1000>(n)
}

#[test]
fn test_main() {
    let result = main(1000);
    assert(result == 0x1caaf75c2b082b66faceefb6b87947ffb3da12fe58cd807f496b703e6c98f8b7);
}
