#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

use json::parse;
use json_core::Outputs;

pub fn main() {
    let data: String = env::read();
    let a: u64 = env::read();
    let b: u64 = env::read();

    if a == 1 || b == 1 {
        panic!("Trivial factors");
    }

    let product = a.checked_mul(b).expect("Integer overflow");
    env::commit(&product);
}
