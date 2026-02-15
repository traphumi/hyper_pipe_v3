use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChunkMeta {
    pub hash: [u8; 32],
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecureFileIndex {
    pub version: u32,
    pub file_name: String,
    pub file_size: u64,
    pub chunks: Vec<ChunkMeta>,
    pub signature: Vec<u8>,
}
