// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `JSON_COMPARE_ELF` and replace
// `METHOD_NAME_ID` with `JSON_COMPARE_ID`
use json_compare_methods::{ JSON_COMPARE_ELF, JSON_COMPARE_ID };
use risc0_zkvm::{ default_executor_from_elf, serde::{ from_slice, to_vec }, ExecutorEnv };
use clap::Parser;
use std::io::{ BufReader, Read };
use json_core::Outputs;

use fuel_core::types::blockchain::block::Block;
use fuel_types::Bytes32;

#[derive(Parser)]
struct Args {
    file1: std::path::PathBuf,
    file2: std::path::PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    // let block: Block;

    let file1 = std::fs::File::open(&args.file1).expect("Could not load first filepath");
    let file2 = std::fs::File::open(&args.file2).expect("Could not load second filepath");

    let mut data1 = String::new();
    BufReader::new(file1).read_to_string(&mut data1).expect("Could not read first file contents");

    let mut data2 = String::new();
    BufReader::new(file2).read_to_string(&mut data2).expect("Could not read second file contents");

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&data1).unwrap())
        .add_input(&to_vec(&data2).unwrap())
        .build()
        .unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, JSON_COMPARE_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(JSON_COMPARE_ID).unwrap();

    // We can extract the output of the journal
    let out: Outputs = from_slice(&receipt.journal).unwrap();

    println!(
        "Successfully read both JSON files with shared value name: {} and equivalences value: {} hash: {}",
        out.shared_value,
        out.val_equivalence,
        out.hash_equivalence
    );
}
