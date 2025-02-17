pub fn main() {
  println!("Profile mode activated: executing bench_fib(100) only...");
  let n = 100;
  let (prove_fib, _verify_fib) = fibonacci_guest::build_fib();
  let (_output, _proof) = prove_fib(n);
  println!("Finished proving");
}
