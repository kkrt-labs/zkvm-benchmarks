fn fib<let N: u32>(n: u32) -> Field {
    let mut a: Field = 0;
    let mut b: Field = 1;

    for i in 1..N {
        if i < n {
            let temp = a;
            a = b;
            b = temp + b;
        }
    }
    b
}

pub fn main(n: u32) -> pub Field {
    fib::<100000>(n)
}

#[test]
fn test_main() {
    let result = main(10);
    assert(result == 0x37);
    let result = main(100000);
    assert(result == 0x21c1d7b08f4286f15846c595a455afa690e6f513c662d3a02c58d00ec98fb7be);
}
