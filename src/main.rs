use ark_serialize::CanonicalDeserialize;
use rlp::{decode, Decodable, DecoderError, Rlp};
use std::convert::TryInto;
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
        println!("size={:?} data={}", rlp.data(), rlp.size());

        let mut buf = rlp.data()?;
        let mut count_buf = [0u8; 4];
        buf.read(&mut count_buf).unwrap();
        let count = u32::from_le_bytes(count_buf);

        for _ in 0..count {
            keys.push([0u8; 32]);
            let keyref = keys.last_mut().unwrap();
            buf.read(keyref).unwrap();
            let mut has_value = [0u8; 1];
            buf.read(&mut has_value).unwrap();
            match has_value[0] {
                0 => values.push(None),
                _ => {
                    values.push(Some([0u8; 32]));
                    match values.last_mut().unwrap() {
                        Some(ref mut valref) => { buf.read(valref).unwrap(); }
                        _ => panic!("invalid value"),
                    }
                }
            }
        }

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
        "de-serialized block:\n- parent hash: {}\n- storage root: {}\n- number: {}\n- key, value list:",
        hex::encode(block.header.parent_hash), hex::encode(block.header.storage_root), hex::encode(block.header.number)
    );

    let keyvals = block.header.keyvals;
    for (i, k) in keyvals.keys.iter().enumerate() {
        match keyvals.values[i] {
            Some(ref val) => println!("\t{} => {}", hex::encode(k), hex::encode(val)),
            None => println!("\t{} is absent", hex::encode(k)),
        }
    }

    let (checked, _) = block
        .header
        .proof
        .verkle_proof
        .check(keyvals.keys, keyvals.values, root);
    if !checked {
        panic!("the proof didn't check")
    }
}
