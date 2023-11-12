use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::peers::Peers;

#[derive(Debug, Clone, Serialize)]
pub struct TrackerRequest {
    peer_id: String,
    port: u16,
    uploaded: usize,
    downloaded: usize,
    left: usize,
    compact: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackerResponse {
    interval: usize,
    peers: Peers,
}

impl TrackerRequest {
    pub fn new(length: usize) -> Self {
        Self {
            peer_id: String::from("AlexanderKalashnikov"),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: length,
            compact: 1,
        }
    }
}

pub fn urlencode_bytes(title: &str, data: &[u8], url: &mut reqwest::Url) {
    let binary = Vec::from_iter(data.iter().copied());
    url.query_pairs_mut()
        .encoding_override(Some(&move |input| {
            if input != "!" {
                // Return the actual value ("info_hash", in this particular case)
                Cow::Borrowed(input.as_bytes())
            } else {
                // When "!" is seen, return the binary data instead
                Cow::Owned(binary.clone())
            }
        }))
        .append_pair(title, "!")
        .finish();
}
