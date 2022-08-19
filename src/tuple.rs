use rlp::{Decodable, DecoderError, Rlp};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Tuple(Vec<u8>, Vec<u8>);

impl Decodable for Tuple {
    fn decode(rlp: &Rlp<'_>) -> std::result::Result<Self, DecoderError> {
        let one = rlp.val_at::<Vec<u8>>(0)?;
        let two = rlp.val_at::<Vec<u8>>(1)?;

        if one.len() != 32 {
            return Err(DecoderError::Custom(
                "Invalid key length in (key, value) tuple",
            ));
        }
        if two.len() != 32 && !two.is_empty() {
            return Err(DecoderError::Custom(
                "Invalid value length in (key, value) tuple",
            ));
        }

        Ok(Tuple(one, two))
    }
}

impl Into<([u8; 32], Option<[u8; 32]>)> for Tuple {
    fn into(self) -> ([u8; 32], Option<[u8; 32]>) {
        // in our case, the output of try_into() will unwrap
        // because the length error will be captured during
        // RLP deserialization. Accordingly, the second element
        // is expecting something that is either a [u32; 8] or
        // something empty, but if a byte array of any other
        // size is passed, it will return `None`.
        (self.0.try_into().unwrap(), self.1.try_into().ok())
    }
}
