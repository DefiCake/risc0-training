#![no_main]

use risc0_zkvm::{ guest::env, sha::{ Impl, Sha256 } };
use json::parse;
use json_core::Outputs;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data: String = env::read();

    let sha = *Impl::hash_bytes(data.as_bytes());
    let json = parse(&data).expect("Could not parse JSON");

    let proven_val: String = json["name"].as_str().unwrap().into();

    let out = Outputs {
        data: proven_val,
        hash: sha,
    };

    env::commit(&out);
}
