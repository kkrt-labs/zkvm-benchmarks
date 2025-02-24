#![no_std]
#![no_main]
zkm2_zkvm::entrypoint!(main);

pub fn main() {
    let n = zkm2_zkvm::io::read::<u32>();

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut sum: u128;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }

    zkm2_zkvm::io::commit(&b);
}
