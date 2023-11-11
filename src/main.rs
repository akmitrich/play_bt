#![allow(dead_code)]
mod hashes;
mod info;

use anyhow::Context;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::info::Torrent;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    c: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Info { torrent: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.c {
        Command::Info { torrent } => {
            let f = std::fs::read(torrent).context("open file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("bencode format")?;
            println!("Info: {:?}", t);
        }
    }
    Ok(println!("Hello, world!"))
}
