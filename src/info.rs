use serde::Deserialize;

use crate::hashes::Hashes;

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Debug, Clone, Deserialize)]
struct Info {
    name: String,
    #[serde(rename = "piece length")]
    plength: usize,
    pieces: Hashes,
    #[serde(flatten)]
    keys: Keys,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Keys {
    SingleFile { length: usize },
    MultiFile { files: Vec<File> },
}

#[derive(Debug, Clone, Deserialize)]
struct File {
    length: usize,
    path: Vec<String>,
}
