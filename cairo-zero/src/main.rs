use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::{path::Path, time::Instant};

use cairo_air::verifier::verify_cairo;
use cairo_air::{CairoProof, PreProcessedTraceVariant};
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use regex::Regex;
use stwo_cairo_adapter::builtins::MemorySegmentAddresses;
use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig, MemoryEntry};
use stwo_cairo_adapter::vm_import::{adapt_to_stwo_input, RelocatedTraceEntry};
use stwo_cairo_adapter::{ProverInput, PublicSegmentContext};
use stwo_cairo_prover::stwo::core::fri::FriConfig;
use stwo_cairo_prover::stwo::core::pcs::PcsConfig;
use stwo_cairo_prover::stwo::core::vcs::blake2_merkle::{
    Blake2sMerkleChannel, Blake2sMerkleHasher,
};
use utils::{
    bench::{benchmark, Metrics},
    metadata::{FIBONACCI_INPUTS, SHA2_INPUTS},
};

// File path constants
const FIBONACCI_SOURCE: &str = "test_data/fibonacci.cairo";
const FIBONACCI_PROGRAM: &str = "test_data/fibonacci.json";
const FIBONACCI_PROOF: &str = "proof_fibonacci.json";
const SHA256_SOURCE: &str = "test_data/sha256.cairo";
const SHA256_PROGRAM: &str = "test_data/sha256.json";
const SHA256_INPUT_TEMPLATE: &str = "test_data/sha256_input_template.json";
const SHA256_INPUT_FILE: &str = "test_data/sha256_input_{}.json";
const SHA256_PROOF: &str = "proof_sha256.json";
const PROOF_DIR: &str = "test_data/proofs";
const KETH_DIR: &str = "keth";

/// Runs a compiled Cairo Zero program and generate a proof of execution.
fn main() {
    dotenv::dotenv().ok();

    // Get benchmark type from environment variable or command line argument
    let bench_type = std::env::var("BENCH_TYPE")
        .unwrap_or_else(|_| std::env::args().nth(1).unwrap_or_else(|| "fib".to_string()));

    match bench_type.as_str() {
        "fib" | "fibonacci" => {
            benchmark(
                bench_cairo_zero_fib,
                &FIBONACCI_INPUTS,
                "../.outputs/benchmark/fib_cairo-zero.csv",
            );
        }
        "sha256" | "sha" => {
            benchmark(
                bench_cairo_zero_sha256,
                &SHA2_INPUTS,
                "../.outputs/benchmark/sha256_cairo-zero.csv",
            );
        }
        _ => {
            eprintln!("Unknown benchmark type: {bench_type}. Use 'fib' or 'sha256'");
            std::process::exit(1);
        }
    }
}

enum ProgramInput {
    File(PathBuf),
    Arguments(String),
}

///
/// Benchmarks using our stwo wrappers via `uv`
///

fn generate_sha256_input(
    size: usize,
    template_path: &Path,
    output_path: &Path,
) -> std::io::Result<()> {
    use std::fs;

    // Read the template file
    let template_content = fs::read_to_string(template_path)?;

    // Generate random bytes
    let bytes = utils::sha2_input(size);

    // Convert bytes to hex string (2 hex chars per byte)
    let hex_text: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();

    // Replace the placeholder with the hex string
    let json_content = template_content.replace("PLACEHOLDER_TEXT", &hex_text);

    // Write to the output file
    fs::write(output_path, json_content)?;

    Ok(())
}

fn bench_cairo_zero_sha256(input_size: usize) -> Metrics {
    let mut metrics = Metrics::new(input_size);

    let workspace_root = std::env::current_dir().expect("Failed to get current directory");
    let keth_path = workspace_root.join(KETH_DIR);
    let source_path = workspace_root.join(SHA256_SOURCE);
    let program_path = workspace_root.join(SHA256_PROGRAM);
    let input_filename = SHA256_INPUT_FILE.replace("{}", &input_size.to_string());
    let input_path = workspace_root.join(&input_filename);
    let template_path = workspace_root.join(SHA256_INPUT_TEMPLATE);
    let proof_dir = workspace_root.join(PROOF_DIR);

    if !keth_path.exists() {
        setup_keth();
    }

    // Generate input file for the specific size
    if let Err(e) = generate_sha256_input(input_size, &template_path, &input_path) {
        eprintln!("Failed to generate SHA256 input file: {}", e);
        std::process::exit(1);
    }

    // See README.md for more details on why we run through a `uv` script.
    if !program_path.exists() {
        compile_cairo_program(&source_path, &program_path, &keth_path);
    }

    let (stdout, peak_mem) = prove_cairo_program(
        &program_path,
        ProgramInput::File(input_path.clone()),
        &proof_dir,
        &keth_path,
    );

    metrics.exec_duration = extract_execution_duration(&stdout);
    metrics.proof_duration = extract_proof_duration(&stdout);
    metrics.peak_memory = peak_mem;

    // Load proof from proof_path to get proof_size and verify it.
    let proof_str =
        std::fs::read_to_string(proof_dir.join(SHA256_PROOF)).expect("Failed to read proof file");
    let proof: CairoProof<Blake2sMerkleHasher> =
        sonic_rs::from_str(&proof_str).expect("Failed to parse proof");
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // Verify.
    let start_time = Instant::now();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    let result = verify_cairo::<Blake2sMerkleChannel>(proof, preprocessed_trace);
    assert!(result.is_ok());
    metrics.verify_duration = start_time.elapsed();

    metrics
}

fn bench_cairo_zero_fib(n: u32) -> Metrics {
    let mut metrics = Metrics::new(n as usize);

    let workspace_root = std::env::current_dir().expect("Failed to get current directory");
    let keth_path = workspace_root.join(KETH_DIR);
    let source_path = workspace_root.join(FIBONACCI_SOURCE);
    let program_path = workspace_root.join(FIBONACCI_PROGRAM);
    let proof_dir = workspace_root.join(PROOF_DIR);

    if !keth_path.exists() {
        setup_keth();
    }

    // See README.md for more details on why we run through a `uv` script.
    if !program_path.exists() {
        compile_cairo_program(&source_path, &program_path, &keth_path);
    }

    let (stdout, peak_mem) = prove_cairo_program(
        &program_path,
        ProgramInput::Arguments(format!("{n}")),
        &proof_dir,
        &keth_path,
    );

    metrics.exec_duration = extract_execution_duration(&stdout);
    metrics.proof_duration = extract_proof_duration(&stdout);
    metrics.peak_memory = peak_mem;

    // Load proof from proof_path to get proof_size and verify it.
    let proof_str = std::fs::read_to_string(proof_dir.join(FIBONACCI_PROOF))
        .expect("Failed to read proof file");
    let proof: CairoProof<Blake2sMerkleHasher> =
        sonic_rs::from_str(&proof_str).expect("Failed to parse proof");
    metrics.proof_bytes = proof.stark_proof.size_estimate();

    // Verify.
    let start_time = Instant::now();
    let preprocessed_trace = PreProcessedTraceVariant::CanonicalWithoutPedersen;
    let result = verify_cairo::<Blake2sMerkleChannel>(proof, preprocessed_trace);
    assert!(result.is_ok());
    metrics.verify_duration = start_time.elapsed();

    metrics
}

/// Configurations for the CSTARK prover.
///
/// Conjecture of n-bit security level: `n = n_queries * log_blowup_factor + pow_bits`.
/// Configuration to achieve 96-bit security level, with PoW bits inferior to 20.
///
/// - The blowup factor greatly influences the proving time.
/// - The number of queries influences the proof size.
/// - The PoW bits influence the proving time, depending on the hardware and the number of bits to grind.
pub const REGULAR_96_BITS: PcsConfig = PcsConfig {
    pow_bits: 16,
    fri_config: FriConfig {
        log_last_layer_degree_bound: 0,
        log_blowup_factor: 1,
        n_queries: 80,
    },
};

///
/// Replacement of the functions from the `cairo-prove` crate that are conflicting with
/// the revision of `stwo` to use.
///

// Deduces the preprocessed trace variant needed for the specific execution, and proves.
pub fn prove(input: ProverInput, pcs_config: PcsConfig) -> CairoProof<Blake2sMerkleHasher> {
    // Currently there are two variants of the preprocessed trace:
    // - Canonical: Pedersen is included in the program.
    // - CanonicalWithoutPedersen: Pedersen is not included in the program.
    // We deduce the variant based on weather the pedersen builtin is included in the program.
    let preprocessed_trace = match input.public_segment_context[1] {
        true => PreProcessedTraceVariant::Canonical,
        false => PreProcessedTraceVariant::CanonicalWithoutPedersen,
    };
    prove_inner(input, preprocessed_trace, pcs_config)
}

fn prove_inner(
    input: ProverInput,
    preprocessed_trace: PreProcessedTraceVariant,
    pcs_config: PcsConfig,
) -> CairoProof<Blake2sMerkleHasher> {
    stwo_cairo_prover::prover::prove_cairo::<Blake2sMerkleChannel>(
        input,
        pcs_config,
        preprocessed_trace,
    )
    .unwrap()
}

pub fn prover_input_from_runner(runner: &CairoRunner) -> ProverInput {
    let public_input = runner.get_air_public_input().unwrap();
    let addresses = public_input
        .public_memory
        .iter()
        .map(|entry| entry.address as u32)
        .collect::<Vec<_>>();
    let segments = public_input
        .memory_segments
        .iter()
        .map(|(&k, v)| {
            (
                k,
                MemorySegmentAddresses {
                    begin_addr: v.begin_addr,
                    stop_ptr: v.stop_ptr,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let trace = runner
        .relocated_trace
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| RelocatedTraceEntry {
            ap: x.ap,
            fp: x.fp,
            pc: x.pc,
        })
        .collect::<Vec<_>>();
    let mem = runner
        .relocated_memory
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            x.as_ref().map(|value| MemoryEntry {
                address: i as u64,
                value: unsafe { std::mem::transmute::<[u8; 32], [u32; 8]>(value.to_bytes_le()) },
            })
        });
    let mem = MemoryBuilder::from_iter(MemoryConfig::default(), mem);
    let main_args = runner
        .get_program()
        .iter_builtins()
        .copied()
        .collect::<Vec<_>>();

    let main_args_slice: &[BuiltinName] = &main_args;
    let public_segment_context = PublicSegmentContext::new(main_args_slice);

    let input =
        adapt_to_stwo_input(&trace, mem, addresses, &segments, public_segment_context).unwrap();
    input
}

///
/// Helper functions
///

fn setup_keth() {
    eprintln!("Keth repository not found. Setting up keth...");

    let mut child = Command::new("sh")
        .arg("setup.sh")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to run setup.sh");

    let status = child.wait().expect("Failed to wait for setup.sh");

    if !status.success() {
        eprintln!("Failed to setup keth environment");
        std::process::exit(1);
    }
}

fn compile_cairo_program(source_path: &Path, program_path: &Path, keth_path: &Path) {
    let child = Command::new("uv")
        .current_dir(&keth_path)
        .arg("run")
        .arg("compile")
        .arg(&source_path)
        .arg("--output-path")
        .arg(&program_path)
        .arg("--proof-mode")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to compile Cairo Zero program");

    let output = child
        .wait_with_output()
        .expect("Failed to get output from compile-cairo");

    if !output.status.success() {
        eprintln!(
            "Failed to compile Cairo Zero program: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }
}

fn prove_cairo_program(
    program_path: &Path,
    input: ProgramInput,
    proof_dir: &Path,
    keth_path: &Path,
    // Returns (stdout, peak_memory_bytes)
) -> (String, usize) {
    let mut cmd = Command::new("uv");
    cmd.current_dir(&keth_path)
        .arg("run")
        .arg("prove-cairo")
        .arg("--compiled-program")
        .arg(&program_path);

    match input {
        ProgramInput::File(input_path) => {
            cmd.arg("--arguments-file").arg(&input_path);
        }
        ProgramInput::Arguments(args) => {
            cmd.arg("--arguments").arg(args);
        }
    }

    let child = cmd
        .arg("--output-dir")
        .arg(&proof_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run `prove-cairo`");

    // Monitor the child process peak RSS using `ps -o rss= -p <pid>`.
    let pid = child.id();
    let peak_kb = Arc::new(AtomicUsize::new(0));
    let stop = Arc::new(AtomicBool::new(false));

    let peak_kb_cl = Arc::clone(&peak_kb);
    let stop_cl = Arc::clone(&stop);
    let monitor = thread::spawn(move || {
        // Poll every 50ms until the root process exits or we are asked to stop.
        while !stop_cl.load(Ordering::Relaxed) && is_pid_alive(pid) {
            if let Some(kb) = get_process_tree_rss_kb(pid) {
                peak_kb_cl.fetch_max(kb, Ordering::Relaxed);
            }
            thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    let output = child
        .wait_with_output()
        .expect("Failed to get output from prove-cairo");

    // Stop monitoring and collect peak.
    stop.store(true, Ordering::Relaxed);
    let _ = monitor.join();
    let peak_bytes = peak_kb.load(Ordering::Relaxed) * 1024; // rss reported in KB

    if !output.status.success() {
        eprintln!(
            "Failed to prove Cairo Zero program: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stdout_clean = strip_ansi_codes(&stdout_str);

    (stdout_clean, peak_bytes)
}

// Returns true if a PID appears to be alive.
fn is_pid_alive(pid: u32) -> bool {
    match Command::new("ps")
        .arg("-p")
        .arg(pid.to_string())
        .arg("-o")
        .arg("pid=")
        .output()
    {
        Ok(out) if out.status.success() => !String::from_utf8_lossy(&out.stdout).trim().is_empty(),
        _ => false,
    }
}

// Sum of RSS (in KB) for the process and all descendants.
fn get_process_tree_rss_kb(root_pid: u32) -> Option<usize> {
    let mut pids = vec![root_pid];
    // Try pgrep-based traversal first (fast), otherwise fallback to parsing full ps output.
    if let Some(tree) = get_descendant_pids_pgrep(root_pid) {
        pids.extend(tree);
    } else if let Some(tree) = get_descendant_pids_via_ps(root_pid) {
        pids.extend(tree);
    }
    get_rss_kb_for_pids(&pids)
}

fn get_descendant_pids_pgrep(root: u32) -> Option<Vec<u32>> {
    use std::collections::VecDeque;
    let mut result = Vec::new();
    let mut q = VecDeque::from([root]);
    while let Some(pid) = q.pop_front() {
        let out = Command::new("pgrep")
            .arg("-P")
            .arg(pid.to_string())
            .output()
            .ok()?;
        if !out.status.success() {
            // pgrep may return non-zero if no children; treat as no children.
            continue;
        }
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if let Ok(child) = line.trim().parse::<u32>() {
                result.push(child);
                q.push_back(child);
            }
        }
    }
    Some(result)
}

fn get_descendant_pids_via_ps(root: u32) -> Option<Vec<u32>> {
    // Build a PPID -> children map from full process table.
    let out = Command::new("ps")
        .arg("-Ao")
        .arg("pid=,ppid=")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let mut map: std::collections::HashMap<u32, Vec<u32>> = std::collections::HashMap::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let mut it = line.split_whitespace();
        let pid = it.next()?.parse::<u32>().ok()?;
        let ppid = it.next()?.parse::<u32>().ok()?;
        map.entry(ppid).or_default().push(pid);
    }
    // BFS descendants
    let mut result = Vec::new();
    let mut stack = vec![root];
    while let Some(p) = stack.pop() {
        if let Some(children) = map.get(&p) {
            for &c in children {
                result.push(c);
                stack.push(c);
            }
        }
    }
    Some(result)
}

fn get_rss_kb_for_pids(pids: &[u32]) -> Option<usize> {
    if pids.is_empty() {
        return Some(0);
    }
    // Try querying in one call with comma-separated PIDs.
    let list = pids
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(",");
    if let Ok(out) = Command::new("ps")
        .arg("-o")
        .arg("rss=")
        .arg("-p")
        .arg(&list)
        .output()
    {
        if out.status.success() {
            let sum = String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|l| l.trim().split_whitespace().next())
                .filter_map(|n| n.parse::<usize>().ok())
                .sum();
            return Some(sum);
        }
    }
    // Fallback: query one by one.
    let mut sum = 0usize;
    for pid in pids {
        if let Ok(out) = Command::new("ps")
            .arg("-o")
            .arg("rss=")
            .arg("-p")
            .arg(pid.to_string())
            .output()
        {
            if out.status.success() {
                if let Some(val) = String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .find_map(|l| l.trim().split_whitespace().next())
                    .and_then(|n| n.parse::<usize>().ok())
                {
                    sum += val;
                }
            }
        }
    }
    Some(sum)
}

fn extract_execution_duration(stdout: &str) -> std::time::Duration {
    let exec_re = Regex::new(
        r"cairo_run_program: vm::vm::runner: close\s+time\.busy=(\d+(?:\.\d+)?)(ms|s|µs)",
    )
    .unwrap();

    if let Some(captures) = exec_re.captures(&stdout) {
        if let Some(duration_str) = captures.get(1) {
            if let Ok(duration_val) = duration_str.as_str().parse::<f64>() {
                let unit = captures.get(2).unwrap().as_str();
                match unit {
                    "ms" => return Duration::from_millis(duration_val as u64),
                    "µs" => return Duration::from_micros(duration_val as u64),
                    "s" => return Duration::from_secs(duration_val as u64),
                    _ => panic!("Unknown unit: {unit}"),
                };
            } else {
                eprintln!("Failed to parse execution duration");
                std::process::exit(1);
            }
        } else {
            eprintln!("Failed to find execution duration in stdout");
            std::process::exit(1);
        }
    } else {
        eprintln!("Failed to find execution duration in stdout when reading {stdout}",);
        std::process::exit(1);
    }
}

fn extract_proof_duration(stdout: &str) -> std::time::Duration {
    // Extract proof duration from stdout using regex
    // Accept ms or s to be robust and reuse stripped output.
    let re = Regex::new(
        r"prove_cairo: stwo_cairo_prover::prover: close\s+time\.busy=(\d+(?:\.\d+)?)(ms|s|μs)",
    )
    .unwrap();

    if let Some(captures) = re.captures(&stdout) {
        if let Some(duration_str) = captures.get(1) {
            if let Ok(duration_val) = duration_str.as_str().parse::<f64>() {
                let unit = captures.get(2).map(|m| m.as_str()).unwrap_or("s");
                let duration_secs = match unit {
                    "ms" => duration_val / 1000.0,
                    "μs" => duration_val / 1_000_000.0,
                    "s" => duration_val,
                    _ => duration_val,
                };
                return std::time::Duration::from_secs_f64(duration_secs);
            } else {
                eprintln!("Failed to parse proof duration");
                std::process::exit(1);
            }
        } else {
            eprintln!("Failed to find proof duration in stdout");
            std::process::exit(1);
        }
    } else {
        eprintln!("Failed to find proof duration in stdout");
        std::process::exit(1);
    }
}

/// Strip ANSI escape codes (colors, styles) from a string so regexes can match reliably.
fn strip_ansi_codes(input: &str) -> String {
    // General CSI sequence matcher: ESC [ ... cmd
    // Matches things like \x1B[0m, \x1B[32m, etc.
    let ansi_re = Regex::new("\x1B\\[[0-?]*[ -/]*[@-~]").unwrap();
    ansi_re.replace_all(input, "").into_owned()
}
