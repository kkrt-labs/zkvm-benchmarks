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
pub fn sha256_hash(n: felt252) -> Array<u8> {
    if n == 32 {
        let data: [u8; 32] = [0x61_u8; 32]; // 'a' repeated 32 times
        sha256(data.span().into())
    } else if n == 64 {
        let data: [u8; 64] = [0x61_u8; 64]; // 'a' repeated 64 times
        sha256(data.span().into())
    } else if n == 96 {
        let data: [u8; 96] = [0x61_u8; 96]; // 'a' repeated 96 times
        sha256(data.span().into())
    } else {
        // Default case
        let data: [u8; 32] = [0x61_u8; 32];
        sha256(data.span().into())
    }
}
