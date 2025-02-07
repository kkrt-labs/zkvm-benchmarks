use k256::{
    ecdsa::{
        signature::Signer,
        Signature, SigningKey,
    },
    elliptic_curve::rand_core::OsRng,
};
use std::{fs::File, io::Write};
use hex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message = b"Hello Public Key!"; 
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    let signature: Signature = signing_key.sign(message);

    let mut message_file = File::create("ecdsa/message.txt")?;
    let mut verifying_key_file = File::create("ecdsa/verifying_key.txt")?;
    let mut signature_file = File::create("ecdsa/signature.txt")?;

    write!(message_file, "{}", hex::encode(message))?;
    write!(
        verifying_key_file,
        "{}",
        hex::encode(verifying_key.to_encoded_point(false).as_bytes())
    )?;
    write!(signature_file, "{}", hex::encode(signature.to_bytes()))?;

    println!("message, verifying_key, and signature have been written to files");
    Ok(())
}
