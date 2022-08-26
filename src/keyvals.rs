use super::tuple::Tuple;
use rlp::{Decodable, DecoderError, Rlp};

pub struct KeyVals {
    pub keys: Vec<[u8; 32]>,
    pub values: Vec<Option<[u8; 32]>>,
}

fn as_val(r: Rlp) -> Result<([u8; 32], Option<[u8; 32]>), DecoderError> {
    Ok(r.as_val::<Tuple>()?.into())
}

impl Decodable for KeyVals {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let result = itertools::process_results(rlp.iter().map(|r| as_val(r)), |iter| iter.unzip());
        let (keys, values): (Vec<[u8; 32]>, Vec<Option<[u8; 32]>>) = result?;

        Ok(KeyVals { keys, values })
    }
}
