use pprof::protos::Message;
use std::fs::File;
use std::io::Write;

pub fn profile_func<F>(func: F, file_path: &str) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(),
{
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso", "rayon::", "std::"])
        .build()
        .unwrap();

    func();

    if let Ok(report) = guard.report().build() {
        let mut file = File::create(file_path)?;
        let profile = report.pprof().unwrap();

        // Serialize the profile to protobuf format
        let mut content = Vec::new();
        profile.encode(&mut content)?;
        file.write_all(&content)?;

        println!("Profile data written to {}", file_path);
    };

    Ok(())
}
