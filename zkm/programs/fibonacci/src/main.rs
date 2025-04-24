#![no_std]
#![no_main]
zkm_zkvm::entrypoint!(main);
use guests::fib;

pub fn main() {
    let n = zkm_zkvm::io::read::<u32>();
    let result = fib::fib(n);
    zkm_zkvm::io::commit(&result);
}
