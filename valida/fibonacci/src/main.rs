use valida_rs::io::read_line;

pub fn main() {
    let n = read_line::<u32>().unwrap();
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut sum: u32;
    for _ in 1..n {
        sum = a + b;
        a = b;
        b = sum;
    }
    println!("{}", b);
}
