use serde::{Deserialize, Serialize};
use sha1::Digest;

use crate::hashes::Hashes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    name: String,
    #[serde(rename = "piece length")]
    plength: usize,
    pieces: Hashes,
    #[serde(flatten)]
    keys: Keys,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Keys {
    SingleFile { length: usize },
    MultiFile { files: Vec<File> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct File {
    length: usize,
    path: Vec<String>,
}

impl Torrent {
    pub fn info(&self) -> &Info {
        &self.info
    }

    pub fn info_hash(&self) -> [u8; 20] {
        let encoded = serde_bencode::to_bytes(self.info()).unwrap();
        let mut hasher = sha1::Sha1::new();
        hasher.update(&encoded);
        hasher.finalize().into()
    }
}
