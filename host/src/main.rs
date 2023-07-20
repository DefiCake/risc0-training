// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `JSON_COMPARE_ELF` and replace
// `METHOD_NAME_ID` with `JSON_COMPARE_ID`
use json_compare_methods::{ JSON_COMPARE_ELF, JSON_COMPARE_ID };
use risc0_zkvm::{ default_executor_from_elf, serde::{ from_slice, to_vec }, ExecutorEnv };
use clap::Parser;
use std::io::{ BufReader, Read };
use json_core::Outputs;

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    let file = std::fs::File::open(&args.path).expect("Could not load filepath");

    let mut data = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Could not read file");

    let env = ExecutorEnv::builder().add_input(&to_vec(&data).unwrap()).build().unwrap();

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
        "Successfully read JSON data with field -name- and value {}, hash {}",
        out.data,
        out.hash
    );
}
