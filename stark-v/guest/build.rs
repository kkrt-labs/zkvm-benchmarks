use std::path::Path;
use std::process::Command;

fn main() {
    // Find where Cargo cached the guest-bin source
    // Use cargo metadata to locate the guest-bin package
    let output = Command::new("cargo")
        .args(["metadata", "--format-version=1"])
        .output()
        .expect("Failed to run cargo metadata");

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("Failed to parse cargo metadata");

    // Find guest-bin package
    let packages = metadata["packages"].as_array().expect("No packages");
    let guest_bin = packages
        .iter()
        .find(|p| p["name"].as_str() == Some("guest-bin"))
        .expect("guest-bin not found in dependencies");

    let manifest_path = guest_bin["manifest_path"]
        .as_str()
        .expect("No manifest_path");
    let guest_bin_dir = Path::new(manifest_path)
        .parent()
        .expect("No parent directory");
    let linker_path = guest_bin_dir.join("linker.ld");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-T{}", linker_path.display());
}
