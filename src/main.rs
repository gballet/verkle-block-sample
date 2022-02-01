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

#[derive(Debug)]
struct Tuple(Vec<u8>, Vec<u8>);

impl Decodable for Tuple {
    fn decode(rlp: &Rlp<'_>) -> std::result::Result<Self, rlp::DecoderError> {
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

struct KeyVals {
    keys: Vec<[u8; 32]>,
    values: Vec<Option<[u8; 32]>>,
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
    let mut file = File::open("block2.rlp").expect("could not open file");
    let mut serialized = Vec::<u8>::new();
    file.read_to_end(&mut serialized).unwrap();

    let block: VerkleBlock = decode(&serialized).expect("could not decode verkle block");

    let parent_root =
        hex::decode("0e494c2da9ce1bc5a09391a2936214f046b4af57722198881a845ee7b2b33486").unwrap();
    let root: EdwardsProjective = CanonicalDeserialize::deserialize(&parent_root[..]).unwrap();

    println!(
        "de-serialized block:\n- parent hash: {}\n- storage root: {}\n- block number: {}\n- key, value list:",
        hex::encode(block.header.parent_hash), hex::encode(block.header.storage_root), hex::encode(block.header.number)
    );

    let keyvals = block.header.keyvals;
    for (k, v) in keyvals.keys.iter().zip(keyvals.values.clone()) {
        match v {
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
