use verkle_trie::proof::VerkleProof;
use rlp::{Decodable, decode, Rlp, DecoderError};
use std::fs::File;
use std::io::Read;

struct VerkleBlock {
}

impl Decodable for VerkleBlock {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        
    }
}

fn main() {
    let mut file = File::open("block1.rlp").expect("could not open file");
    let mut serialized = Vec::<u8>::new();
    file.read_to_end(&mut serialized);

    let block : VerkleBlock = decode(&serialized).expect("could not decode verkle block");
}
