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
    type Error = <Vec<u8> as TryInto<[u8; 32]>>::Error;
    fn try_into(
        self,
    ) -> Result<
        ([u8; 32], Option<[u8; 32]>),
        Self::Error,
    > {
        let mut second = None;

        if self.1.len() > 0 {
            second = Some(self.1.try_into()?);
        }

        Ok((self.0.try_into()?, second))
    }
}
