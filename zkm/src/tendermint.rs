use std::error::Error;
use std::{fs::File, io::Read};
use std::time::{Duration, Instant};

use tendermint_light_client_verifier::{
    options::Options, types::LightBlock, ProdVerifier, Verdict, Verifier,
};

use zkm_sdk::include_elf;
use zkm_sdk::{ProverClient, ZKMStdin};

use utils::size;

const TENDERMINT_ELF: &[u8] = include_elf!("tendermint");

pub fn bench_tendermint(_v: u32) -> (Duration, usize, u64) {
    // Load light blocks from the `files` subdirectory
    let (light_block_1, light_block_2) = get_light_blocks();

    let expected_verdict = verify_blocks(light_block_1.clone(), light_block_2.clone());

    let encoded_1 = serde_cbor::to_vec(&light_block_1).unwrap();
    let encoded_2 = serde_cbor::to_vec(&light_block_2).unwrap();

    let mut stdin = ZKMStdin::new();
    stdin.write_vec(encoded_1);
    stdin.write_vec(encoded_2);

    // TODO: normally we could just write the LightBlock, but bincode doesn't work with LightBlock.
    // The following code will panic.
    // let encoded: Vec<u8> = bincode::serialize(&light_block_1).unwrap();
    // let decoded: LightBlock = bincode::deserialize(&encoded[..]).unwrap();

    let client = ProverClient::new();
    let (pk, vk) = client.setup(TENDERMINT_ELF);

    println!("benchmark_tendermint start");
    let start = Instant::now();
    let proof = client.prove(&pk, stdin.clone()).run().expect("proving failed");
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("benchmark_tendermint end, duration: {:?}", duration.as_secs_f64());

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Verify the public values
    let mut expected_public_values: Vec<u8> = Vec::new();
    expected_public_values.extend(light_block_1.signed_header.header.hash().as_bytes());
    expected_public_values.extend(light_block_2.signed_header.header.hash().as_bytes());
    expected_public_values.extend(serde_cbor::to_vec(&expected_verdict).unwrap());

    assert_eq!(proof.public_values.as_ref(), expected_public_values);

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (_, report) = client.execute(TENDERMINT_ELF, stdin).run().expect("proving failed");
    println!("executed program with {} cycles", report.total_instruction_count());

    (duration, size(&proof), report.total_instruction_count())
}

fn get_light_blocks() -> (LightBlock, LightBlock) {
    let light_block_1 = load_light_block(2279100).expect("Failed to generate light block 1");
    let light_block_2 = load_light_block(2279130).expect("Failed to generate light block 2");
    (light_block_1, light_block_2)
}

fn verify_blocks(light_block_1: LightBlock, light_block_2: LightBlock) -> Verdict {
    let vp = ProdVerifier::default();
    let opt = Options {
        trust_threshold: Default::default(),
        trusting_period: Duration::from_secs(500),
        clock_drift: Default::default(),
    };
    let verify_time = light_block_2.time() + Duration::from_secs(20);
    vp.verify_update_header(
        light_block_2.as_untrusted_state(),
        light_block_1.as_trusted_state(),
        &opt,
        verify_time.unwrap(),
    )
}

pub fn load_light_block(block_height: u64) -> Result<LightBlock, Box<dyn Error>> {
    let mut file = File::open(format!("files/block_{}.json", block_height))?;
    let mut block_response_raw = String::new();
    file.read_to_string(&mut block_response_raw)
        .unwrap_or_else(|_| panic!("Failed to read block number {}", block_height));
    Ok(serde_json::from_str(&block_response_raw)?)
}
