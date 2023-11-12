use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const HANDSHAKE_SIZE: usize = std::mem::size_of::<HandShake>();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(C)]
pub struct HandShake {
    length: u8,
    bittorrent: [u8; 19],
    reserved: [u8; 8],
    info_hash: [u8; 20],
    peer_id: [u8; 20],
}

impl HandShake {
    const BITTORRENT: usize = 1;
    const RESERVED: usize = 20;
    const INFO_HASH: usize = 28;
    const PEER_ID: usize = 48;
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            length: 19,
            bittorrent: *b"BitTorrent protocol",
            reserved: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub async fn send(&self, stream: &mut TcpStream) -> anyhow::Result<()> {
        let mut bytes = [0; HANDSHAKE_SIZE];
        bytes[0] = self.length;
        bytes_copy(&self.bittorrent, &mut bytes[Self::BITTORRENT..]);
        bytes_copy(&self.reserved, &mut bytes[Self::RESERVED..]);
        bytes_copy(&self.info_hash, &mut bytes[Self::INFO_HASH..]);
        bytes_copy(&self.peer_id, &mut bytes[Self::PEER_ID..]);
        stream.write_all(&bytes).await.context("send handshake")
    }

    pub async fn recv(stream: &mut TcpStream) -> anyhow::Result<Self> {
        let mut bytes = [0; HANDSHAKE_SIZE];
        let _ = stream.read_exact(&mut bytes).await?;
        let mut result = Self::new([0; 20], [0; 20]);
        result.length = bytes[0];
        bytes_copy(
            &bytes[Self::BITTORRENT..Self::RESERVED],
            &mut result.bittorrent,
        );
        bytes_copy(
            &bytes[Self::RESERVED..Self::INFO_HASH],
            &mut result.reserved,
        );
        bytes_copy(
            &bytes[Self::INFO_HASH..Self::PEER_ID],
            &mut result.info_hash,
        );
        bytes_copy(&bytes[Self::PEER_ID..], &mut result.peer_id);
        Ok(result)
    }
}

fn bytes_copy(src: &[u8], mut dst: &mut [u8]) {
    use std::io::Write;
    let _ = dst.write(src);
}
