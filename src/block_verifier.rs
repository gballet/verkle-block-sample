use ark_serialize::CanonicalDeserialize;
use block_verkle_proof_extractor::{Element, VerkleBlock};
use clap::Parser;
use rlp::decode;
use std::fs::File;
use std::io::Read;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "block2.rlp")]
    filename: String,

    #[clap(
        short,
        long,
        default_value = "323ce96c53ff0abf906b68e544885ca9798d0e042b690b76eefb7e9d9866db68"
    )]
    parent_root: String,
}

fn main() {
    let args = Args::parse();

    let mut file = File::open(args.filename).expect("could not open file");
    let mut serialized = Vec::<u8>::new();
    file.read_to_end(&mut serialized).unwrap();

    let block: VerkleBlock = decode(&serialized).expect("could not decode verkle block");

    let parent_root = hex::decode(args.parent_root).unwrap();
    let root: Element = CanonicalDeserialize::deserialize(&parent_root[..]).unwrap();

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

    println!("Proof verified");
}
