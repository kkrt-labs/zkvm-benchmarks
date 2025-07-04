use noir_fib::fib;

pub fn main(n: u32) -> pub Field {
    fib::<100000>(n)
}

#[test]
fn test_main() {
    let result = main(100000);
    assert(result == 0x21c1d7b08f4286f15846c595a455afa690e6f513c662d3a02c58d00ec98fb7be);
}
