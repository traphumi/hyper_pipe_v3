mod cli;
mod chunker;
mod crypto;
mod downloader;
mod index;
mod types;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    match cli::parse()? {
        cli::Command::Keygen => crypto::generate_keys()?,
        cli::Command::Index { file, key } => index::build_index(&file, &key)?,
        cli::Command::Download { url, idx, key, out } => {
            downloader::secure_download(&url, &idx, &key, &out).await?
        }
    }
    Ok(())
}
