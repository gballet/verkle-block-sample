use rlp::{Decodable, DecoderError, Rlp};
use std::convert::TryInto;

#[derive(Debug)]
pub struct Tuple(Vec<u8>, Vec<u8>);

impl Decodable for Tuple {
    fn decode(rlp: &Rlp<'_>) -> std::result::Result<Self, DecoderError> {
        Ok(Tuple(rlp.val_at::<Vec<u8>>(0)?, rlp.val_at::<Vec<u8>>(1)?))
    }
}

impl TryInto<([u8; 32], Option<[u8; 32]>)> for Tuple {
    type Error = String;
    fn try_into(
        self,
    ) -> std::result::Result<
        ([u8; 32], Option<[u8; 32]>),
        <Self as TryInto<([u8; 32], Option<[u8; 32]>)>>::Error,
    > {
        let mut second = None;

        if self.1.len() > 0 {
            // pad up the values with 0s until the length is 32
            // will fail if len() > 32, should not happen for a
            // somewhat valid input.
            let mut padded = [0u8; 32];
            padded[..self.1.len()].copy_from_slice(&self.1[..]);
            second = Some(padded);
        }

        Ok((self.0.try_into().unwrap(), second))
    }
}
