use guests::fib::fib;
use valida_rs::io::read_line;

pub fn main() {
    let n = read_line::<u32>().unwrap();
    let result = fib(n);
    println!("{}", result);
}
