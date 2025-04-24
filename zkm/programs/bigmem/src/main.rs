#![no_main]

use core::hint::black_box;
zkm_zkvm::entrypoint!(main);

pub fn main() {
    let value = zkm_zkvm::io::read::<u32>();

    let array = [value; 128000];
    black_box(array);
    let result = array[16000];

    zkm_zkvm::io::commit(&result);
}
