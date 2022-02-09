use super::tuple::Tuple;
use rlp::{Decodable, DecoderError, Rlp};
use std::convert::TryInto;

pub struct KeyVals {
    pub keys: Vec<[u8; 32]>,
    pub values: Vec<Option<[u8; 32]>>,
}

impl Decodable for KeyVals {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let (keys, values): (Vec<[u8; 32]>, Vec<Option<[u8; 32]>>) = rlp
            .iter()
            .map(|r| r.as_val::<Tuple>().unwrap().try_into().unwrap())
            .unzip();

        Ok(KeyVals {
            keys: keys,
            values: values,
        })
    }
}
