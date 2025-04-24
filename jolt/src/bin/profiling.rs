use fibonacci_guest;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting profiling...");

    // Set Fibonacci input
    let n = 10;
    println!("Computing Fibonacci({}) with profiling...", n);

    // Get the Fibonacci function from the guest code
    let (prove_fib, _verify_fib) = fibonacci_guest::build_fib();

    // Create a pprof guard with 100 Hz sampling rate
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso", "rayon", "std"])
        .build()
        .unwrap();

    let (output, _proof) = prove_fib(n);

    if let Ok(report) = guard.report().build() {
        let mut file = File::create("profile.pb").unwrap();
        let profile = report.pprof().unwrap();

        let mut content = Vec::new();
        profile.encode(&mut content).unwrap();
        file.write_all(&content).unwrap();
    };

    Ok(())
}
