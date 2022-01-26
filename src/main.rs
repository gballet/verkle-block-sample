use ark_serialize::CanonicalDeserialize;
use rlp::{decode, Decodable, DecoderError, Rlp};
use std::fs::File;
use std::io::Read;
use verkle_trie::proof::VerkleProof;
use verkle_trie::EdwardsProjective;

struct Proof {
    verkle_proof: VerkleProof,
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

struct KeyVals {
    keys: Vec<[u8; 32]>,
    values: Vec<Option<[u8; 32]>>,
}

impl Decodable for KeyVals {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let mut keys: Vec<[u8; 32]> = Vec::new();
        let mut values: Vec<Option<[u8; 32]>> = Vec::new();

        //let mut offset = 0usize;
        //while offset < keyvals.len() {
        //keys.push([0u8; 32]);
        //let keyref = keys.last_mut().unwrap();
        //keyref[..].clone_from_slice(&keyvals[offset..offset + 32]);
        //offset += 32;
        //match keyvals[offset] {
        //0 => values.push(None),
        //_ => {
        //values.push(Some([0u8; 32]));
        //match values.last_mut().unwrap() {
        //Some(ref mut valref) => {
        //valref[..].clone_from_slice(&keyvals[offset + 1..offset + 33])
        //}
        //_ => panic!("invalid value"),
        //}
        //offset += 33;
        //}
        //}
        //}

        Ok(KeyVals {
            keys: keys,
            values: values,
        })
    }
}

struct VerkleHeader {
    parent_hash: Vec<u8>,
    storage_root: Vec<u8>,
    number: Vec<u8>,
    proof: Proof,
    keyvals: KeyVals,
}

impl Decodable for VerkleHeader {
    fn decode(rlp: &rlp::Rlp<'_>) -> Result<Self, rlp::DecoderError> {
        Ok(VerkleHeader {
            parent_hash: rlp.at(0)?.as_val::<Vec<u8>>()?,
            storage_root: rlp.at(3)?.as_val::<Vec<u8>>()?,
            number: rlp.at(8)?.as_val::<Vec<u8>>()?,
            proof: rlp.at(16)?.as_val::<Proof>()?,
            keyvals: rlp.at(17)?.as_val::<KeyVals>()?,
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

    let parent_root =
        hex::decode("0e494c2da9ce1bc5a09391a2936214f046b4af57722198881a845ee7b2b33486").unwrap();
    let root: EdwardsProjective = CanonicalDeserialize::deserialize(&parent_root[..]).unwrap();

    println!(
        "de-serialized block:\n- parent hash: {:x?}\n- storage root: {:x?}\n- number: {:x?}",
        block.header.parent_hash, block.header.storage_root, block.header.number
    )

    //let kvp = block.header.keyvals_and_proof;
    //let (checked, _) = kvp.verkle_proof.check(kvp.keys, kvp.values, root);
    //if !checked {
    //panic!("the proof didn't check")
    //}
}
