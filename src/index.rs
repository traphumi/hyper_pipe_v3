use crate::{chunker, crypto, types::SecureFileIndex, types::ChunkMeta};
use anyhow::Result;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn build_index(file_path: &str, priv_key_path: &str) -> Result<()> {
    let mut file = fs::File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let priv_key = crypto::load_signing_key(priv_key_path)?;
    let chunk_offsets = chunker::get_chunks(&buffer);

    let mut chunks = Vec::new();
    for (offset, size) in chunk_offsets {
        let data = &buffer[offset..offset + size];
        let hash = blake3::hash(data).into();
        chunks.push(ChunkMeta { hash, size: size as u32 });
    }

    let mut index = SecureFileIndex {
        version: 3,
        file_name: Path::new(file_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        file_size: buffer.len() as u64,
        chunks,
        signature: vec![],
    };

    let serialized = serde_json::to_vec(&index)?;
    index.signature = crypto::sign(&serialized, &priv_key).to_vec();

    fs::write(format!("{}.idx", file_path), serde_json::to_string_pretty(&index)?)?;
    println!("✅ Index created: {}.idx", file_path);

    Ok(())
}
