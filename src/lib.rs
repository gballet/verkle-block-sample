pub use verkle_trie::Element;
use rlp::{Decodable, DecoderError, Rlp};

pub mod keyvals;
pub mod proof;
pub(crate) mod tuple;

pub struct VerkleHeader {
    pub parent_hash: Vec<u8>,
    pub storage_root: Vec<u8>,
    pub number: Vec<u8>,
    pub proof: proof::Proof,
    pub keyvals: keyvals::KeyVals,
}

impl Decodable for VerkleHeader {
    fn decode(rlp: &rlp::Rlp<'_>) -> Result<Self, rlp::DecoderError> {
        Ok(VerkleHeader {
            parent_hash: rlp.at(0)?.as_val::<Vec<u8>>()?,
            storage_root: rlp.at(3)?.as_val::<Vec<u8>>()?,
            number: rlp.at(8)?.as_val::<Vec<u8>>()?,
            proof: rlp.at(16)?.as_val::<proof::Proof>()?,
            keyvals: rlp.at(17)?.as_val::<keyvals::KeyVals>()?,
        })
    }
}

pub struct VerkleBlock {
    pub header: VerkleHeader,
}

impl Decodable for VerkleBlock {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        let headerrlp = rlp.at(0)?;
        let header: VerkleHeader = VerkleHeader::decode(&headerrlp)?;
        Ok(VerkleBlock { header })
    }
}

#[cfg(test)]
mod test {

    use bytebuffer::ByteBuffer;
    use std::convert::TryInto;
    use verkle_trie::database::memory_db::MemoryDb;
    // Note: for this to work, TestConfig needs to be made
    // public in the verkle-trie crate.
    use verkle_trie::{trie::Trie, Fr, TestConfig, TrieTrait};

    fn scalar_to_array(scalar: &Fr) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        use ark_serialize::CanonicalSerialize;
        scalar.serialize_uncompressed(&mut bytes[..]).unwrap();
        bytes
    }

    #[test]
    fn compare_with_geth() {
        let db = MemoryDb::new();
        let mut trie = Trie::new(TestConfig::new(db));

        let keys: Vec<[u8; 32]> = vec![
            hex::decode("318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d01")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526400")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a02")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d02")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a04")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526402")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526403")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a00")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a03")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526401")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e6ed6c222e3985050b4fc574b136b0a42c63538e9ab970995cd418ba8e526404")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d00")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("18fb432d3b859ec3a1803854e8cceea75d092e52d0d4a4398d13022496745a01")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        let values = vec![
            hex::decode("320122e8584be00d000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0300000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("1bc176f2790c91e6000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e703000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        let absent_keys = vec![
            hex::decode("318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d03")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("318dea512b6f3237a2d4763cf49bf26de3b617fb0cabe38a97807a5549df4d04")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        for (idx, key) in keys.iter().enumerate() {
            trie.insert_single(key.clone(), values[idx]);
        }
        let root_hash = trie.root_hash();
        println!("root hash = {:?}", hex::encode(scalar_to_array(&root_hash)));
        let vp = trie.create_verkle_proof(
            keys.clone()
                .into_iter()
                .chain(absent_keys.clone().into_iter()),
        );
        let mut buffer = ByteBuffer::new();
        vp.write(&mut buffer).unwrap();
        println!("serialized proof={}", hex::encode(buffer.to_bytes()));

        let (valid, _) = vp.check(
            keys.into_iter()
                .chain(absent_keys.clone().into_iter())
                .collect(),
            values
                .iter()
                .map(|v| {
                    Some(
                        v.to_vec()
                            .try_into()
                            .expect("could not convert value into [u8; 32]"),
                    )
                })
                .chain(absent_keys.iter().map(|_| None))
                .collect(),
            trie.root_commitment(),
        );
        assert!(valid);
    }

    #[test]
    fn condrieu_block_283() {
        let db = MemoryDb::new();
        let mut trie = Trie::new(TestConfig::new(db));

        let keys: Vec<[u8; 32]> = vec![
            hex::decode("6766d007d8fd90ea45b2ac9027ff04fa57e49527f11010a12a73f58ffa580800")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("6766d007d8fd90ea45b2ac9027ff04fa57e49527f11010a12a73f58ffa580801")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("6766d007d8fd90ea45b2ac9027ff04fa57e49527f11010a12a73f58ffa580802")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("6766d007d8fd90ea45b2ac9027ff04fa57e49527f11010a12a73f58ffa580803")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("6766d007d8fd90ea45b2ac9027ff04fa57e49527f11010a12a73f58ffa580804")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9f2a59ea98d7cb610eff49447571e1610188937ce9266c6b4ded1b6ee37ecd00")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9f2a59ea98d7cb610eff49447571e1610188937ce9266c6b4ded1b6ee37ecd01")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9f2a59ea98d7cb610eff49447571e1610188937ce9266c6b4ded1b6ee37ecd02")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9f2a59ea98d7cb610eff49447571e1610188937ce9266c6b4ded1b6ee37ecd03")
                .unwrap()
                .try_into()
                .unwrap(),
        ];
        let values = vec![
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("cbf0245862edde191e0200000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0900000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e0f8d1ffa7cc525fe23b00000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0200000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        let absent_keys = vec![
            hex::decode("0b373ba3992dde5cfee854e1a786559ba0b6a13d376550c1ed58c00dc9706f00")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0b373ba3992dde5cfee854e1a786559ba0b6a13d376550c1ed58c00dc9706f02")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0b373ba3992dde5cfee854e1a786559ba0b6a13d376550c1ed58c00dc9706f03")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0b373ba3992dde5cfee854e1a786559ba0b6a13d376550c1ed58c00dc9706f75")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0b373ba3992dde5cfee854e1a786559ba0b6a13d376550c1ed58c00dc9706f80")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0bfc1ece8fb78ece3cba37063d607e9801a50509feb7fc7533522f6670418800")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0bfc1ece8fb78ece3cba37063d607e9801a50509feb7fc7533522f6670418802")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0bfc1ece8fb78ece3cba37063d607e9801a50509feb7fc7533522f6670418803")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("91faf3920905ef6a9f7b9c57ff3e49dbb8727271955061c26f2fb220ba6dfd00")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("91faf3920905ef6a9f7b9c57ff3e49dbb8727271955061c26f2fb220ba6dfd01")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("91faf3920905ef6a9f7b9c57ff3e49dbb8727271955061c26f2fb220ba6dfd02")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("91faf3920905ef6a9f7b9c57ff3e49dbb8727271955061c26f2fb220ba6dfd03")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("91faf3920905ef6a9f7b9c57ff3e49dbb8727271955061c26f2fb220ba6dfd04")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8300")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8301")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8302")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8303")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8304")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8380")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8381")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8382")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8383")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("9b5c07351e1938b60443e64a7fbda33533355ccc3b9b1eb64b653b78d09c8384")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        for (idx, key) in keys.iter().enumerate() {
            trie.insert_single(key.clone(), values[idx]);
        }

        // this key is present in the tree, but its presence isn't being proven
        trie.insert_single(
            hex::decode("9f2a59ea98d7cb610eff49447571e1610188937ce9266c6b4ded1b6ee37ecd04")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
        );

        let root_hash = trie.root_hash();
        println!("root hash = {:?}", hex::encode(scalar_to_array(&root_hash)));
        let vp = trie.create_verkle_proof(
            keys.clone()
                .into_iter()
                .chain(absent_keys.clone().into_iter()),
        );
        let mut bytes = Vec::new();
        vp.write(&mut bytes).unwrap();
        println!("{}", hex::encode(bytes));

        let (checked, _) = vp.check(
            keys.into_iter()
                .chain(absent_keys.clone().into_iter())
                .collect(),
            values
                .iter()
                .map(|v| {
                    Some(
                        v.to_vec()
                            .try_into()
                            .expect("could not convert value into [u8; 32]"),
                    )
                })
                .chain(absent_keys.iter().map(|_| None))
                .collect(),
            trie.root_commitment(),
        );
        assert!(checked);
    }

    #[test]
    fn condrieu_block_1190() {
        let db = MemoryDb::new();
        let mut trie = Trie::new(TestConfig::new(db));

        let keys: Vec<[u8; 32]> = vec![
            hex::decode("694dc6ea427eea992996e5dfa6992eb0434d7e305574cf74f45226538e34a800")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("69596485f622c118c15dd39dbeb6187996d925687f61966e19e3339d90140a03")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e4acdc8de380f2b37191ccafa9f9dadf70d48ea24ee3cb5b866f1e5b1d3c615d")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e4f1872b1681742a8d126266a635dd9a4f6c560cb7ae2707805f304e5df60b00")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        let values = vec![
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        let absent_keys = vec![
            hex::decode("695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf902")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("e4f88f8c09077a0f433ce5873ae741ede1ff4577cc4198b3a4a0f880a6cc8f03")
                .unwrap()
                .try_into()
                .unwrap(),
        ];

        for (idx, key) in keys.iter().enumerate() {
            trie.insert_single(key.clone(), values[idx]);
        }
        let _root_hash = trie.root_hash();
        let vp = trie.create_verkle_proof(absent_keys.clone().into_iter());
        let mut bytes = Vec::new();
        vp.write(&mut bytes).unwrap();
    }
}
