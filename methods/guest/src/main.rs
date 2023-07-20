#![no_main]

use risc0_zkvm::{ guest::env, sha::{ Impl, Sha256 } };
use json::parse;
use json_core::Outputs;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let file1: String = env::read();
    let sha1 = *Impl::hash_bytes(file1.as_bytes());
    let json1 = parse(&file1).expect("Could not parse JSON");

    let file2: String = env::read();
    let sha2 = *Impl::hash_bytes(file2.as_bytes());
    let json2 = parse(&file2).expect("Could not parse second JSON");

    let val1: String = json1["name"].as_str().unwrap().into();
    let val2: String = json2["name"].as_str().unwrap().into();

    let val_equivalence = val1 == val2;
    let sha_equivalence = sha1 == sha2;

    let out = Outputs {
        shared_value: val1,
        val_equivalence,
        hash_equivalence: sha_equivalence,
    };

    env::commit(&out);
}
