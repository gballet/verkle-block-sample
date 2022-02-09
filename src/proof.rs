use rlp::{Decodable, DecoderError, Rlp};
use verkle_trie::proof::VerkleProof;

pub(crate) struct Proof {
    pub verkle_proof: VerkleProof,
}

impl Decodable for Proof {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let serialized_proof = rlp.data()?;
        let proof = VerkleProof::read(&serialized_proof[..]).unwrap();

        Ok(Proof {
            verkle_proof: proof,
        })
    }
}

