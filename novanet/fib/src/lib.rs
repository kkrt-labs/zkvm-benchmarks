#![no_main]

#[no_mangle]
pub fn fib(n: u32) -> u128 {
    guests::fib::fib(n)
}
