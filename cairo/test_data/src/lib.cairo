#[feature("deprecated-sha256")]
use alexandria_math::sha256::sha256;

#[executable]
pub fn main(n_felt: felt252) -> felt252 {
    let mut a = 0;
    let mut b = 1;
    let n: u32 = n_felt.try_into().unwrap();
    for _ in 1..n {
        let temp = a;
        a = b;
        b += temp;
    }
    b.into()
}

#[executable]
pub fn sha256_hash(data: Array<u8>) -> Array<u8> {
    sha256(data)
}
