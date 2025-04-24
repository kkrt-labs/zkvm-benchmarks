use utils::profile::profile_func;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting profiling...");

    // Set Fibonacci input
    let n = 10;
    println!("Computing Fibonacci({}) with profiling...", n);

    // Get the Fibonacci function from the guest code
    let (prove_fib, _verify_fib) = fibonacci_guest::build_fib();

    profile_func(
        || {
            let (_output, _proof) = prove_fib(n);
        },
        "../profile_outputs/profile_jolt.pb",
    )?;

    println!("Profiling complete!");
    Ok(())
}
