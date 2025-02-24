use std::time::{Duration, Instant};
use std::fs::File;
use std::io::BufReader;
use std::ops::Range;

use plonky2::field::goldilocks_field::GoldilocksField;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use plonky2::util::timing::TimingTree;

use zkm_emulator::utils::{load_elf_with_patch, split_prog_into_segs};
use zkm_prover::all_stark::AllStark;
use zkm_prover::config::StarkConfig;
use zkm_prover::cpu::kernel::assembler::segment_kernel;
use zkm_prover::fixed_recursive_verifier::AllRecursiveCircuits;
use zkm_prover::proof;
use zkm_prover::prover::prove;
use zkm_prover::verifier::verify_proof;

use k256::{
    ecdsa::Signature,
    elliptic_curve::sec1::EncodedPoint,
    Secp256k1,
};

const FIBONACCI_ELF: &str = "./programs/fibonacci/elf/mips-zkm-zkvm-elf";
const SHA2_ELF: &str = "./programs/sha2/elf/mips-zkm-zkvm-elf";
const SHA2_CHAIN_ELF: &str = "./programs/sha2-chain/elf/mips-zkm-zkvm-elf";
const SHA3_CHAIN_ELF: &str = "./programs/sha3-chain/elf/mips-zkm-zkvm-elf";
const SHA3_ELF: &str = "./programs/sha3/elf/mips-zkm-zkvm-elf";
const BIGMEM_ELF: &str = "./programs/bigmem/elf/mips-zkm-zkvm-elf";
const ECDSA_ELF: &str = "./programs/ecdsa/elf/mips-zkm-zkvm-elf";
const SEG_SIZE: usize = 262144 * 8; // G
type BenchResult = (Duration, usize, usize);

const MESSAGE: &[u8] = include_bytes!("../../helper/ecdsa_signature/message.txt");
const KEY: &[u8] = include_bytes!("../../helper/ecdsa_signature/verifying_key.txt");
const SIGNATURE: &[u8] = include_bytes!("../../helper/ecdsa_signature/signature.txt");

const DEGREE_BITS_RANGE: [Range<usize>; 12] = [
    10..21,
    12..22,
    11..21,
    8..21,
    6..10,
    6..10,
    6..16,
    6..16,
    6..16,
    6..16,
    6..21,
    13..25,
];

pub fn init_logger() {
    let logl = std::env::var("RUST_LOG").unwrap_or("info".to_string());
    std::env::set_var("RUST_LOG", &logl);
    env_logger::init()
}

pub fn benchmark_sha2_chain(iters: u32) -> BenchResult {
    let input = [5u8; 32];
    let mut state = load_elf_with_patch(SHA2_CHAIN_ELF, vec![]);
    state.add_input_stream(&input);
    state.add_input_stream(&iters);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha2-chain";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_sha2_chain start, iters: {}", iters);
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2_chain end, duration: {:?}", duration.as_secs_f64());

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size, total_steps)
}

pub fn benchmark_sha3_chain(iters: u32) -> BenchResult {
    let input = [5u8; 32];
    let mut state = load_elf_with_patch(SHA3_CHAIN_ELF, vec![]);
    state.add_input_stream(&input);
    state.add_input_stream(&iters);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha3-chain";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_sha3_chain start, iters: {}", iters);
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3_chain end, duration: {:?}", duration.as_secs_f64());

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size, total_steps)
}

pub fn benchmark_sha2(num_bytes: usize) -> BenchResult {
    let input = vec![5u8; num_bytes];
    let mut state = load_elf_with_patch(SHA2_ELF, vec![]);
    state.add_input_stream(&input);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha2";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_sha2 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2 end, duration: {:?}", duration.as_secs_f64());

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size, total_steps)
}

pub fn benchmark_sha3(num_bytes: usize) -> BenchResult {
    let input = vec![5u8; num_bytes];
    let mut state = load_elf_with_patch(SHA3_ELF, vec![]);
    state.add_input_stream(&input);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/sha3";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_sha3 start, num_bytes: {}", num_bytes);
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha3 end, duration: {:?}", duration.as_secs_f64());

    let _hash =  state.read_public_values::<[u8; 32]>();

    (duration, size, total_steps)
}

pub fn benchmark_fibonacci(n: u32) -> BenchResult {
    let mut state = load_elf_with_patch(FIBONACCI_ELF, vec![]);
    state.add_input_stream(&n);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/fibonacci";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_fibonacci start, n: {}", n);
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_fibonacc end, duration: {:?}", duration.as_secs_f64());

    let _output = state.read_public_values::<u128>();
    (duration, size, total_steps)
}

pub fn benchmark_bigmem(value: u32) -> BenchResult {
    let mut state = load_elf_with_patch(BIGMEM_ELF, vec![]);
    state.add_input_stream(&value);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/bigmem";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_bigmem start, value: {}", value);
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_bigmem end, duration: {:?}", duration.as_secs_f64());

    let _output = state.read_public_values::<u32>();
    (duration, size, total_steps)
}

pub fn benchmark_ecdsa(_length: usize) -> BenchResult {
    let message = hex::decode(MESSAGE).expect("Failed to decode hex of 'message'");

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        &hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");
    let input = (encoded_point, message, signature);
    let mut state = load_elf_with_patch(ECDSA_ELF, vec![]);
    state.add_input_stream(&input);

    let seg_size = SEG_SIZE;
    let seg_path = "/tmp/zkm/ecdsa";

    let (total_steps, seg_num, mut state) = split_prog_into_segs(state, seg_path, "", seg_size);

    println!("benchmark_ecdsa start");
    let start = Instant::now();
    let size = if seg_num == 1 {
        let seg_file = format!("{seg_path}/{}", 0);
        prove_single_seg_common(&seg_file, "", "", "")
    } else {
        prove_multi_seg_common(seg_path, "", "", "", seg_num, 0)
    };
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_sha2 end, duration: {:?}", duration.as_secs_f64());

    let _is_ok =  state.read_public_values::<[u8; 32]>();

    (duration, size, total_steps)
}

fn prove_single_seg_common(
    seg_file: &str,
    basedir: &str,
    block: &str,
    file: &str,
) -> usize {
    let seg_reader = BufReader::new(File::open(seg_file).unwrap());
    let kernel = segment_kernel(basedir, block, file, seg_reader);

    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let allstark: AllStark<F, D> = AllStark::default();
    let config = StarkConfig::standard_fast_config();
    let mut timing = TimingTree::new("prove", log::Level::Info);
    let allproof: proof::AllProof<GoldilocksField, C, D> =
        prove(&allstark, &kernel, &config, &mut timing).unwrap();
    let mut count_bytes: usize = 0;
    for (row, proof) in allproof.stark_proofs.clone().iter().enumerate() {
        let proof_str = serde_json::to_string(&proof.proof).unwrap();
        log::info!("row:{} proof bytes:{}", row, proof_str.len());
        count_bytes += proof_str.len();
    }
    timing.filter(Duration::from_millis(100)).print();
    log::info!("total proof bytes:{}KB", count_bytes / 1024);
    verify_proof(&allstark, allproof, &config).unwrap();
    log::info!("Prove done");
    count_bytes
}

fn prove_multi_seg_common(
    seg_dir: &str,
    basedir: &str,
    block: &str,
    file: &str,
    seg_file_number: usize,
    seg_start_id: usize,
) -> usize {
    type F = GoldilocksField;
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;

    if seg_file_number < 2 {
        panic!("seg file number must >= 2\n");
    }

    let total_timing = TimingTree::new("prove total time", log::Level::Info);
    let all_stark = AllStark::<F, D>::default();
    let config = StarkConfig::standard_fast_config();
    // Preprocess all circuits.
    let all_circuits =
        AllRecursiveCircuits::<F, C, D>::new(&all_stark, &DEGREE_BITS_RANGE, &config);

    let seg_file = format!("{}/{}", seg_dir, seg_start_id);
    log::info!("Process segment {}", seg_file);
    let seg_reader = BufReader::new(File::open(seg_file).unwrap());
    let input_first = segment_kernel(basedir, block, file, seg_reader);
    let mut timing = TimingTree::new("prove root first", log::Level::Info);
    // let (mut agg_proof, mut updated_agg_public_values) =
    let mut agg_receipt =
        all_circuits.prove_root(&all_stark, &input_first, &config, &mut timing).unwrap();

    timing.filter(Duration::from_millis(100)).print();
    all_circuits.verify_root(agg_receipt.clone()).unwrap();

    let mut base_seg = seg_start_id + 1;
    let mut seg_num = seg_file_number - 1;
    let mut is_agg = false;
    let assumptions = vec![];

    if seg_file_number % 2 == 0 {
        let seg_file = format!("{}/{}", seg_dir, seg_start_id + 1);
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(seg_file).unwrap());
        let input = segment_kernel(basedir, block, file, seg_reader);
        timing = TimingTree::new("prove root second", log::Level::Info);
        let receipt = all_circuits.prove_root_with_assumption(
            &all_stark,
            &input,
            &config,
            &mut timing,
            assumptions.clone(),
        ).unwrap();
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_root(receipt.clone()).unwrap();

        timing = TimingTree::new("prove aggression", log::Level::Info);
        // We can duplicate the proofs here because the state hasn't mutated.
        agg_receipt = all_circuits.prove_aggregation(false, &agg_receipt, false, &receipt).unwrap();
        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_aggregation(&agg_receipt).unwrap();

        is_agg = true;
        base_seg = seg_start_id + 2;
        seg_num -= 1;
    }

    for i in 0..seg_num / 2 {
        let seg_file = format!("{}/{}", seg_dir, base_seg + (i << 1));
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(&seg_file).unwrap());
        let input_first = segment_kernel(basedir, block, file, seg_reader);
        let mut timing = TimingTree::new("prove root first", log::Level::Info);
        let root_receipt_first = all_circuits.prove_root_with_assumption(
            &all_stark,
            &input_first,
            &config,
            &mut timing,
            assumptions.clone(),
        ).unwrap();

        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_root(root_receipt_first.clone()).unwrap();

        let seg_file = format!("{}/{}", seg_dir, base_seg + (i << 1) + 1);
        log::info!("Process segment {}", seg_file);
        let seg_reader = BufReader::new(File::open(&seg_file).unwrap());
        let input = segment_kernel(basedir, block, file, seg_reader);
        let mut timing = TimingTree::new("prove root second", log::Level::Info);
        let root_receipt = all_circuits.prove_root_with_assumption(
            &all_stark,
            &input,
            &config,
            &mut timing,
            assumptions.clone(),
        ).unwrap();
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_root(root_receipt.clone()).unwrap();

        timing = TimingTree::new("prove aggression", log::Level::Info);
        // We can duplicate the proofs here because the state hasn't mutated.
        let new_agg_receipt =
            all_circuits.prove_aggregation(false, &root_receipt_first, false, &root_receipt).unwrap();
        timing.filter(Duration::from_millis(100)).print();
        all_circuits.verify_aggregation(&new_agg_receipt).unwrap();

        timing = TimingTree::new("prove nested aggression", log::Level::Info);

        // We can duplicate the proofs here because the state hasn't mutated.
        agg_receipt =
            all_circuits.prove_aggregation(is_agg, &agg_receipt, true, &new_agg_receipt).unwrap();
        is_agg = true;
        timing.filter(Duration::from_millis(100)).print();

        all_circuits.verify_aggregation(&agg_receipt).unwrap();
    }

    let block_receipt =
        all_circuits.prove_block(None, &agg_receipt).unwrap();

    let size = serde_json::to_string(&block_receipt).unwrap().len();
    log::info!(
        "proof size: {:?}",
        size
    );
    total_timing.filter(Duration::from_millis(100)).print();
    size
}
