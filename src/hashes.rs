use serde::{de::Visitor, Deserialize};

#[derive(Debug, Clone)]
pub struct Hashes(Vec<[u8; 20]>);
struct HashStrVisitor;
impl<'de> Visitor<'de> for HashStrVisitor {
    type Value = Hashes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a byte string whose length is a multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!("Unacceptable length {}", v.len())));
        }
        Ok(Hashes(
            v.chunks_exact(20)
                .map(|slice_20| slice_20.try_into().expect("length is guaranteed to be 20"))
                .collect(),
        ))
    }
}

impl<'de> Deserialize<'de> for Hashes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashStrVisitor)
    }
}
