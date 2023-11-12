#![allow(dead_code)]

use anyhow::Context;
use clap::{Parser, Subcommand};
use play_bt::{
    info::Torrent,
    peer::HandShake,
    tracker::{urlencode_bytes, TrackerRequest, TrackerResponse},
};
use std::path::PathBuf;
use tokio::net::TcpStream;

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    c: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Info { torrent: PathBuf },
    InfoHash { torrent: PathBuf },
    Peers { torrent: PathBuf },
    Handshake { torrent: PathBuf, peer: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.c {
        Command::Info { torrent } => {
            let f = std::fs::read(torrent).context("open file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("bencode format")?;
            println!("Info: {:?}", t);
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
        Command::Peers { torrent } => {
            let f = std::fs::read(torrent).context("open file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("bencode format")?;
            let r = TrackerRequest::new(t.info().length());
            let mut tracker_url = reqwest::Url::parse(t.announce()).context("parse announce")?;
            tracker_url.set_query(
                serde_urlencoded::to_string(r)
                    .ok()
                    .as_ref()
                    .map(|s| s.as_str()),
            );
            urlencode_bytes("info_hash", &t.info_hash(), &mut tracker_url);
            println!("Tracker URL: {:?}", tracker_url);
            let resp = reqwest::get(tracker_url)
                .await
                .context("send request to tracker")?;
            let tracker_resp: TrackerResponse =
                serde_bencode::from_bytes(&resp.bytes().await.context("response bytes")?)
                    .context("bencode decoding")?;
            println!("Tracker response: {:?}", tracker_resp);
        }
        Command::Handshake { torrent, peer } => {
            let f = std::fs::read(torrent).context("open file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("bencode format")?;
            let h = HandShake::new(t.info_hash(), *b"AlexanderKalashnikov");
            let mut peer = TcpStream::connect(peer).await.context("connect to peer")?;
            h.send(&mut peer).await?;
            let answer = HandShake::recv(&mut peer).await?;
            println!("Handshake: {:?} -> {:?}", h, answer);
        }
    }
    Ok(println!("Hello, world!"))
}
