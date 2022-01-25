use ark_serialize::CanonicalDeserialize;
use rlp::{decode, Decodable, DecoderError, Rlp};
use std::fs::File;
use std::io::Read;
use verkle_trie::proof::VerkleProof;
use verkle_trie::EdwardsProjective;

struct VerkleKeysValsAndProofs {
    verkle_proof: VerkleProof,
    keys: Vec<[u8; 32]>,
    values: Vec<Option<[u8; 32]>>,
}

impl Decodable for VerkleKeysValsAndProofs {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let serialized_proof = rlp.list_at::<u8>(0)?;
        let proof = VerkleProof::read(&serialized_proof[..]).unwrap();

        let keyvals: Vec<u8> = rlp.list_at(1)?;
        let mut keys: Vec<[u8; 32]> = Vec::new();
        let mut values: Vec<Option<[u8; 32]>> = Vec::new();

        let mut offset = 0usize;
        while offset < keyvals.len() {
            keys.push([0u8; 32]);
            let keyref = keys.last_mut().unwrap();
            keyref[..].clone_from_slice(&keyvals[offset..offset + 32]);
            offset += 32;
            match keyvals[offset] {
                0 => values.push(None),
                _ => {
                    values.push(Some([0u8; 32]));
                    match values.last_mut().unwrap() {
                        Some(ref mut valref) => {
                            valref[..].clone_from_slice(&keyvals[offset + 1..offset + 33])
                        }
                        _ => panic!("invalid value"),
                    }
                    offset += 33;
                }
            }
        }

        Ok(VerkleKeysValsAndProofs {
            verkle_proof: proof,
            keys: keys,
            values: values,
        })
    }
}

struct VerkleHeader {
    parent_hash: Vec<u8>,
    storage_root: Vec<u8>,
    number: Vec<u8>,
    keyvals_and_proof: VerkleKeysValsAndProofs,
}

impl Decodable for VerkleHeader {
    fn decode(rlp: &rlp::Rlp<'_>) -> Result<Self, rlp::DecoderError> {
        let serialized_proof_rlp = rlp.at(16)?;
        let kvandproofs = VerkleKeysValsAndProofs::decode(&serialized_proof_rlp)?;
        Ok(VerkleHeader {
            parent_hash: rlp.list_at(0).unwrap(),
            storage_root: rlp.list_at(3).unwrap(),
            number: rlp.list_at(8).unwrap(),
            keyvals_and_proof: kvandproofs,
        })
    }
}

struct VerkleBlock {
    header: VerkleHeader,
}

impl Decodable for VerkleBlock {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let headerrlp = rlp.at(0)?;
        let header: VerkleHeader = VerkleHeader::decode(&headerrlp)?;
        Ok(VerkleBlock { header: header })
    }
}

fn main() {
    let mut file = File::open("block1.rlp").expect("could not open file");
    let mut serialized = Vec::<u8>::new();
    file.read_to_end(&mut serialized).unwrap();

    let block: VerkleBlock = decode(&serialized).expect("could not decode verkle block");

    let parent_root = hex::decode("").unwrap();
    let root: EdwardsProjective = CanonicalDeserialize::deserialize(&parent_root[..]).unwrap();

    let kvp = block.header.keyvals_and_proof;
    let (checked, _) = kvp.verkle_proof.check(kvp.keys, kvp.values, root);
    if !checked {
        panic!("the proof didn't check")
    }
}
