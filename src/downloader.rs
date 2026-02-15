use crate::{crypto, types::SecureFileIndex};
use anyhow::{anyhow, Result};
use std::fs;
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub async fn secure_download(
    url: &str,
    index_path: &str,
    pub_key_path: &str,
    output_dir: &str,
) -> Result<()> {
    let index_data = fs::read(index_path)?;
    let mut index: SecureFileIndex = serde_json::from_slice(&index_data)?;

    let pub_key = crypto::load_verifying_key(pub_key_path)?;
    let sig = index.signature.clone();
    index.signature.clear();

    let serialized = serde_json::to_vec(&index)?;
    crypto::verify(&serialized, &sig, &pub_key)?;

    let output_path = PathBuf::from(output_dir).join(&index.file_name);
    if let Some(p) = output_path.parent() {
        fs::create_dir_all(p)?;
    }

    let client = reqwest::Client::new();
    let mut file = OpenOptions::new().create(true).write(true).open(&output_path).await?;

    let mut offset = 0u64;
    for chunk in index.chunks {
        let end = offset + chunk.size as u64 - 1;
        let resp = client
            .get(url)
            .header("Range", format!("bytes={}-{}", offset, end))
            .send()
            .await?
            .bytes()
            .await?;

        if blake3::hash(&resp).as_bytes() != &chunk.hash {
            return Err(anyhow!("Chunk hash mismatch"));
        }

        file.write_all(&resp).await?;
        offset += chunk.size as u64;
    }

    println!("🎯 Download complete");
    Ok(())
}
