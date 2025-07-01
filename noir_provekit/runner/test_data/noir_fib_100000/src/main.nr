use noir_fib::fib;

pub fn main() -> pub Field {
    fib::<100000>()
}

#[test]
fn test_main() {
    let result = main();
    assert(result == 0x21c1d7b08f4286f15846c595a455afa690e6f513c662d3a02c58d00ec98fb7be);
}
