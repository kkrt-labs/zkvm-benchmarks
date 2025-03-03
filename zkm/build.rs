use zkm_build::BuildArgs;

fn main() {
    let guest_paths = vec![
        "bigmem",
        "fibonacci",
        "sha2",
        "sha2-chain",
        "sha3",
        "sha3-chain",
        "ecdsa",
        "transfer-eth",
    ];
    for guest in guest_paths {
        build_guest(guest);
    }
}

fn build_guest(guest: &str) {
    let args = BuildArgs {
        output_directory: format!(
            "{}/programs/{}/elf",
            env!("CARGO_MANIFEST_DIR"), guest,
        ),
        ..Default::default()
    };
    let guest_path = format!(
        "{}/programs/{}",
        env!("CARGO_MANIFEST_DIR"), guest,
    );
    zkm_build::build_program_with_args(&guest_path, args);
    let guest_target_path = format!(
        "{}/{}/{}",
        guest_path,
        zkm_build::DEFAULT_OUTPUT_DIR,
        zkm_build::BUILD_TARGET
    );
    println!("cargo:rustc-env=GUEST_TARGET_PATH={}", guest_target_path);
}
