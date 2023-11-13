use anyhow::Ok;
use bytes::BytesMut;
use tokio_codec::{Decoder, Encoder};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MessageTag {
    Bitfield = 5,
}

#[derive(Debug, Clone)]
pub struct Message {
    tag: MessageTag,
    payload: Vec<u8>,
}

struct MessageDecoder;

impl Decoder for MessageDecoder {
    type Item = Message;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 5 {
            return Ok(None);
        }
        let mut length_arr = [0; 4];
        length_arr.copy_from_slice(&src[..4]);
        let length = u32::from_be_bytes(length_arr) as usize;
        if src.len() < 4 + length {
            src.reserve(4 + length - src.len());
            return Ok(None);
        }
        let tag = src[4];
        let data = src[5..4 + length].to_vec();
        Ok(Some(Message {
            tag: MessageTag::Bitfield,
            payload: data,
        }))
    }
}

struct MessageEncoder;

impl Encoder for MessageEncoder {
    type Item = Message;

    type Error = anyhow::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}
