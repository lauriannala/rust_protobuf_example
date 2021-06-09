
use std::io::Cursor;

use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/protobuf.items.rs"));
}

pub fn create_message(query: String) -> items::Message {
    let mut message = items::Message::default();
    message.query = query;
    message
}

pub fn serialize_message(message: &items::Message) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(message.encoded_len());

    message.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_message(buf: &[u8]) -> Result<items::Message, prost::DecodeError> {
    items::Message::decode(&mut Cursor::new(buf))
}
