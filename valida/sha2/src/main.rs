use valida_rs::io::read;
use guests::sha2::sha2;

pub fn main() {
    // Read all input bytes from stdin until EOF
    let input = read().expect("Failed to read input");
    let result = sha2(&input);
    println!("{:?}", result);
}