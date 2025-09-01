use valida_rs::io::read_line;
use guests::sha2::sha2;

pub fn main() {
    let num_bytes = read_line::<usize>().unwrap();
    let input = vec![0u8; num_bytes];
    let result = sha2(&input);
    println!("{:?}", result);
}