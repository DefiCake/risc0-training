// TODO: Update the name of the method loaded by the prover. E.g., if the method
// is `multiply`, replace `METHOD_NAME_ELF` with `JSON_ELF` and replace
// `METHOD_NAME_ID` with `JSON_ID`
use json_methods::{ JSON_ELF, JSON_ID };
use risc0_zkvm::{ default_executor_from_elf, serde::{ from_slice, to_vec }, ExecutorEnv };
use clap::Parser;
use std::io::{ BufReader, Read };

#[derive(Parser)]
struct Args {
    path: std::path::PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    let file = std::fs::File::open(&args.path).expect("Could not load filepath");

    // First, we construct an executor environment
    let a: u64 = 17;
    let b: u64 = 23;

    let mut data = String::new();
    BufReader::new(file).read_to_string(&mut data).expect("Could not read file");

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&data).unwrap())
        .add_input(&to_vec(&a).unwrap())
        .add_input(&to_vec(&b).unwrap())
        .build()
        .unwrap();

    // TODO: add guest input to the executor environment using
    // ExecutorEnvBuilder::add_input().
    // To access this method, you'll need to use the alternate construction
    // ExecutorEnv::builder(), which creates an ExecutorEnvBuilder. When you're
    // done adding input, call ExecutorEnvBuilder::build().

    // For example:
    // let env = ExecutorEnv::builder().add_input(&vec).build().unwrap();

    // Next, we make an executor, loading the (renamed) ELF binary.
    let mut exec = default_executor_from_elf(env, JSON_ELF).unwrap();

    // Run the executor to produce a session.
    let session = exec.run().unwrap();

    // Prove the session to produce a receipt.
    let receipt = session.prove().unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(JSON_ID).unwrap();

    // We can extract the output of the journal, c = a * b
    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);
    println!("Successfully read JSON data {}", data);
}
