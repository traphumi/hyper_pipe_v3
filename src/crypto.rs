use anyhow::{anyhow, Context, Result};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey, Signature};
use rand::rngs::OsRng;
use std::fs;

pub fn generate_keys() -> Result<()> {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();

    fs::write("private.key", signing_key.to_bytes())?;
    fs::write("public.key", verifying_key.to_bytes())?;
    println!("✅ Keypair generated");

    Ok(())
}

pub fn load_signing_key(path: &str) -> Result<SigningKey> {
    let bytes = fs::read(path).context("Private key not found")?;
    let array: [u8; 32] = bytes.try_into().map_err(|_| anyhow!("Invalid private key size"))?;
    Ok(SigningKey::from_bytes(&array))
}

pub fn load_verifying_key(path: &str) -> Result<VerifyingKey> {
    let bytes = fs::read(path).context("Public key not found")?;
    let array: [u8; 32] = bytes.try_into().map_err(|_| anyhow!("Invalid public key size"))?;
    Ok(VerifyingKey::from_bytes(&array)?)
}

pub fn sign(data: &[u8], key: &SigningKey) -> Signature {
    key.sign(data)
}

pub fn verify(data: &[u8], sig: &[u8], key: &VerifyingKey) -> Result<()> {
    let sig = Signature::from_slice(sig)?;
    key.verify(data, &sig)?;
    Ok(())
}
