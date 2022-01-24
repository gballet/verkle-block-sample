use rlp::{decode, Decodable, DecoderError, Rlp};
use std::fs::File;
use std::io::Read;
use verkle_trie::proof::VerkleProof;

struct VerkleKeysValsAndProofs {
    verkle_proof: VerkleProof,
    keys: Vec<Vec<u8>>,
    values: Vec<Option<Vec<u8>>>,
}

impl Decodable for VerkleKeysValsAndProofs {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        todo!();
    }
}

struct VerkleHeader {
    keyvals_and_proof: VerkleKeysValsAndProofs,
}

impl Decodable for VerkleHeader {
    fn decode(rlp: &rlp::Rlp<'_>) -> Result<Self, rlp::DecoderError> {
    }
}

struct VerkleBlock {
    header: VerkleHeader,
}

impl Decodable for VerkleBlock {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        
    }
}

fn main() {
    let mut file = File::open("block1.rlp").expect("could not open file");
    let mut serialized = Vec::<u8>::new();
    file.read_to_end(&mut serialized).unwrap();

    let block: VerkleBlock = decode(&serialized).expect("could not decode verkle block");

    let kvp = block.header.keyvals_and_proof;
    //let (checked, _) = kvp.verkle_proof.check(kvp.keys, kvp.values);
    //if !checked {
        //panic!("the proof didn't check")
    //}
}
