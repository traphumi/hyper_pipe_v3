use anyhow::{anyhow, Result};

pub enum Command {
    Keygen,
    Index { file: String, key: String },
    Download { url: String, idx: String, key: String, out: String },
}

pub fn parse() -> Result<Command> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("keygen") => Ok(Command::Keygen),
        Some("index") => Ok(Command::Index {
            file: args.get(2).ok_or(anyhow!("Missing file"))?.clone(),
            key: args.get(3).ok_or(anyhow!("Missing key"))?.clone(),
        }),
        Some("download") => Ok(Command::Download {
            url: args.get(2).ok_or(anyhow!("Missing url"))?.clone(),
            idx: args.get(3).ok_or(anyhow!("Missing index"))?.clone(),
            key: args.get(4).ok_or(anyhow!("Missing key"))?.clone(),
            out: args.get(5).ok_or(anyhow!("Missing output"))?.clone(),
        }),
        _ => Err(anyhow!("Invalid command")),
    }
}
