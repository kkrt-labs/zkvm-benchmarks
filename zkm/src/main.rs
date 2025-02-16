use std::time::{Duration, Instant};

use utils::benchmark;

use anyhow;
use std::fs::File;
use std::io::BufReader;
use std::ops::Range;

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::timing::TimingTree;
use plonky2x::backend::circuit::Groth16WrapperParameters;
use plonky2x::backend::wrapper::wrap::WrappedCircuit;
use plonky2x::frontend::builder::CircuitBuilder as WrapperBuilder;
use plonky2x::prelude::DefaultParameters;

use zkm_emulator::utils::{load_elf_with_patch, split_prog_into_segs};
use zkm_prover::all_stark::AllStark;
use zkm_prover::config::StarkConfig;
use zkm_prover::cpu::kernel::assembler::segment_kernel;
use zkm_prover::fixed_recursive_verifier::AllRecursiveCircuits;
use zkm_prover::generation::state::{AssumptionReceipts, Receipt};
use zkm_prover::proof;
use zkm_prover::proof::PublicValues;
use zkm_prover::prover::prove;
use zkm_prover::verifier::verify_proof;

type BenchResult = (Duration, usize, usize);

const FIBONACCI_ELF: &str = "./fibonacci/target/mips-unknown-linux-musl/release/fibonacci";
const SHA2_ELF: &str = "./sha2/target/mips-unknown-linux-musl/release/sha2-bench";
const SHA2_CHAIN_ELF: &str = "./sha2-chain/target/mips-unknown-linux-musl/release/sha2-chain";
const SHA3_CHAIN_ELF: &str = "./sha3-chain/target/mips-unknown-linux-musl/release/sha3-chain";
const SHA3_ELF: &str = "./sha3/target/mips-unknown-linux-musl/release/sha3-bench";
const BIGMEM_ELF: &str = "./bigmem/target/mips-unknown-linux-musl/release/bigmem";
const SEG_SIZE: usize = 262144 * 8; //G

const DEGREE_BITS_RANGE: [Range<usize>; 8] =
    [10..21, 12..22, 11..21, 8..21, 6..21, 6..21, 6..21, 13..23];
const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

fn main() {
    init_logger();

    let _ = std::fs::remove_dir_all("/tmp/zkm.old");
    let _ = std::fs::rename("/tmp/zkm", "/tmp/zkm.old");

    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--once") {
        once_fibonacci();
    } else {
        let lengths = [32, 256, 512, 1024, 2048];
        benchmark(
            benchmark_sha2,
            &lengths,
            "../benchmark_outputs/sha2_zkm.csv",
            "byte length",
        );
        benchmark(
            benchmark_sha3,
            &lengths,
            "../benchmark_outputs/sha3_zkm.csv",
            "byte length",
        );

        let ns = [100, 1000, 10000, 50000];
        benchmark(
            benchmark_fibonacci,
            &ns,
            "../benchmark_outputs/fiboancci_zkm.csv",
            "n",
        );

        let values = [5];
        benchmark(
            benchmark_bigmem,
            &values,
            "../benchmark_outputs/bigmem_zkm.csv",
            "value",
        );

        let iters = [230, 460, 920, 1840, 3680];
        benchmark(
            benchmark_sha2_chain,
            &iters,
            "../benchmark_outputs/sha2_chain_zkm.csv",
            "iters",
        );
        benchmark(
            benchmark_sha3_chain,
            &iters,
            "../benchmark_outputs/sha3_chain_zkm.csv",
            "iters",
        );
    }
}

pub fn prove_segments(
    seg_dir: &str,
    basedir: &str,
    block: &str,
    file: &str,
    seg_file_number: usize,
    seg_start_id: usize,
    assumptions: AssumptionReceipts<F, C, D>,
) -> anyhow::Result<usize> {
    type InnerParameters = DefaultParameters;
    type OuterParameters = Groth16WrapperParameters;

    let total_timing = TimingTree::new("prove total time", log::Level::Info);
    let all_stark = AllStark::<F, D>::default();
    let config = StarkConfig::standard_fast_config();
    // Preprocess all circuits.
    let all_circuits =
        AllRecursiveCircuits::<F, C, D>::new(&all_stark, &DEGREE_BITS_RANGE, &config);

    let seg_file = format!("{}/{}", seg_dir, seg_start_id);
    log::info!("Process segment {}", seg_file);
    let seg_reader = BufReader::new(File::open(seg_file)?);
    let input_first = segment_kernel(basedir, block, file, seg_reader);
    let mut timing = TimingTree::new("prove root first", log::Level::Info);
    let mut agg_receipt = all_circuits.prove_root_with_assumption(
        &all_stark,
        &input_first,
        &config,
        &mut timing,
        assumptions.clone(),
    )?;

    timing.filter(Duration::from_millis(100)).print();
    all_circuits.verify_root(agg_receipt.clone())?;

    let mut base_seg = seg_start_id + 1;
    let mut seg_num = seg_file_number - 1;
    let mut is_agg = false;

    if seg_file_number % 2 == 0 {
        let seg_file = format!("{}/{}", seg_dir, seg_start_id + 1);
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(seg_file)?);
        let input = segment_kernel(basedir, block, file, seg_reader);
        timing = TimingTree::new("prove root second", log::Level::Info);
        let receipt = all_circuits.prove_root_with_assumption(
            &all_stark,
            &input,
            &config,
            &mut timing,
            assumptions.clone(),
        )?;
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_root(receipt.clone())?;

        timing = TimingTree::new("prove aggression", log::Level::Info);
        // We can duplicate the proofs here because the state hasn't mutated.
        agg_receipt = all_circuits.prove_aggregation(false, &agg_receipt, false, &receipt)?;
        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_aggregation(&agg_receipt)?;

        is_agg = true;
        base_seg = seg_start_id + 2;
        seg_num -= 1;
    }

    for i in 0..seg_num / 2 {
        let seg_file = format!("{}/{}", seg_dir, base_seg + (i << 1));
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(&seg_file)?);
        let input_first = segment_kernel(basedir, block, file, seg_reader);
        let mut timing = TimingTree::new("prove root first", log::Level::Info);
        let root_receipt_first = all_circuits.prove_root_with_assumption(
            &all_stark,
            &input_first,
            &config,
            &mut timing,
            assumptions.clone(),
        )?;

        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_root(root_receipt_first.clone())?;

        let seg_file = format!("{}/{}", seg_dir, base_seg + (i << 1) + 1);
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(&seg_file)?);
        let input = segment_kernel(basedir, block, file, seg_reader);
        let mut timing = TimingTree::new("prove root second", log::Level::Info);
        let root_receipt = all_circuits.prove_root_with_assumption(
            &all_stark,
            &input,
            &config,
            &mut timing,
            assumptions.clone(),
        )?;
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_root(root_receipt.clone())?;

        timing = TimingTree::new("prove aggression", log::Level::Info);
        // We can duplicate the proofs here because the state hasn't mutated.
        let new_agg_receipt =
            all_circuits.prove_aggregation(false, &root_receipt_first, false, &root_receipt)?;
        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_aggregation(&new_agg_receipt)?;

        timing = TimingTree::new("prove nested aggression", log::Level::Info);

        // We can duplicate the proofs here because the state hasn't mutated.
        agg_receipt =
            all_circuits.prove_aggregation(is_agg, &agg_receipt, true, &new_agg_receipt)?;
        is_agg = true;
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_aggregation(&agg_receipt)?;
    }

    log::info!(
        "proof size: {:?}",
        serde_json::to_string(&agg_receipt.proof().proof)
            .unwrap()
            .len()
    );
    let final_receipt = if seg_file_number > 1 {
        let block_receipt = all_circuits.prove_block(None, &agg_receipt)?;
        all_circuits.verify_block(&block_receipt)?;
        let build_path = "../verifier/data".to_string();
        let path = format!("{}/test_circuit/", build_path);
        let builder = WrapperBuilder::<DefaultParameters, 2>::new();
        let mut circuit = builder.build();
        circuit.set_data(all_circuits.block.circuit);
        let mut bit_size = vec![32usize; 16];
        bit_size.extend(vec![8; 32]);
        bit_size.extend(vec![64; 68]);
        let wrapped_circuit = WrappedCircuit::<InnerParameters, OuterParameters, D>::build(
            circuit,
            Some((vec![], bit_size)),
        );
        let wrapped_proof = wrapped_circuit.prove(&block_receipt.proof()).unwrap();
        wrapped_proof.save(path).unwrap();

        block_receipt
    } else {
        agg_receipt
    };

    log::info!("build finish");

    total_timing.filter(Duration::from_millis(100)).print();
    // Ok(final_receipt)
    let size = serde_json::to_string(&final_receipt.proof()).unwrap().len();
    Ok(size)
}

fn init_logger() {
    let logl = std::env::var("RUST_LOG").unwrap_or("info".to_string());
    std::env::set_var("RUST_LOG", &logl);
    env_logger::init()
}

fn benchmark_sha2_chain(iters: u32) -> BenchResult {
    let input = [5u8; 32];
    let mut state = load_elf_with_patch(SHA2_CHAIN_ELF, vec![]);
    state.add_input_stream(&input);
    state.add_input_stream(&iters);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha2-chain";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let start = Instant::now();
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = state.read_public_values::<[u8; 32]>();

    (duration, size, state.cycle as usize)
}

fn benchmark_sha3_chain(iters: u32) -> BenchResult {
    let input = [5u8; 32];
    let mut state = load_elf_with_patch(SHA3_CHAIN_ELF, vec![]);
    state.add_input_stream(&input);
    state.add_input_stream(&iters);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha3-chain";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let start = Instant::now();
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = state.read_public_values::<[u8; 32]>();

    (duration, size, state.cycle as usize)
}

fn benchmark_sha2(num_bytes: usize) -> BenchResult {
    let input = vec![5u8; num_bytes];
    let mut state = load_elf_with_patch(SHA2_ELF, vec![]);
    state.add_input_stream(&input);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha2";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let start = Instant::now();
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = state.read_public_values::<[u8; 32]>();

    (duration, size, state.cycle as usize)
}

fn benchmark_sha3(num_bytes: usize) -> BenchResult {
    let input = vec![5u8; num_bytes];
    let mut state = load_elf_with_patch(SHA3_ELF, vec![]);
    state.add_input_stream(&input);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha3";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let start = Instant::now();
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _hash = state.read_public_values::<[u8; 32]>();

    (duration, size, state.cycle as usize)
}

fn once_fibonacci() {
    println!("Profile mode activated: executing bench_fib(100) only...");
    let n = 100u32;
    let mut state = load_elf_with_patch(FIBONACCI_ELF, vec![]);
    state.add_input_stream(&n);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/fibonacci";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("Proving...");
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    println!("Done!");
}

fn benchmark_fibonacci(n: u32) -> BenchResult {
    let mut state = load_elf_with_patch(FIBONACCI_ELF, vec![]);
    state.add_input_stream(&n);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/fibonacci";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let start = Instant::now();
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _output = state.read_public_values::<u128>();
    (duration, size, state.cycle as usize)
}

fn benchmark_bigmem(value: u32) -> BenchResult {
    let mut state = load_elf_with_patch(BIGMEM_ELF, vec![]);
    state.add_input_stream(&value);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/bigmem";

    let (_total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    let start = Instant::now();
    let size = prove_segments(&seg_path, "", "", "", seg_num, 0, vec![]).unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);

    let _output = state.read_public_values::<u32>();
    (duration, size, state.cycle as usize)
}
