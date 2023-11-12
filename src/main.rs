#![allow(dead_code)]
mod hashes;
mod info;

use crate::info::Torrent;
use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    c: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Info { torrent: PathBuf },
    InfoHash { torrent: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.c {
        Command::Info { torrent } => {
            let f = std::fs::read(torrent).context("open file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("bencode format")?;
            println!("Info: {:?}", t.info());
        }
        Command::InfoHash { torrent } => {
            let f = std::fs::read(torrent).context("open file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("bencode format")?;
            let encoded = serde_bencode::to_bytes(t.info())?;
            println!(
                "Info raw: {:?}",
                encoded.iter().copied().map(char::from).collect::<Vec<_>>()
            );
            println!("Info Hash: {}", hex::encode(t.info_hash().as_slice()));
        }
    }
    Ok(println!("Hello, world!"))
}
