#![no_std]
#![no_main]

use core::hint::black_box;

zkm_runtime::entrypoint!(main);

pub fn main() {
    let value = zkm_runtime::io::read::<u32>();

    let array = [value; 128000];
    black_box(array);
    let result = array[16000];

    zkm_runtime::io::commit(&result);
}
