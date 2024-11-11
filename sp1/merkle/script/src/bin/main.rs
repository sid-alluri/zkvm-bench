use clap::Parser;
use serde_json::to_string_pretty;
use sha3::{Digest, Keccak256};
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::Instant;
/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");
#[derive(serde::Serialize)]
struct BenchResults {
    framework: String,
    task: String,
    execution_time: f64,
    setup_time: f64,
    prover_time: f64,
    verifier_time: f64,
    proof_size: u64,
    num_cycles: u64,
    num_mem_access: u64,
    speed: f64,
}
/// The arguments for the command.
#[derive(Parser)]
#[command(name = "task")]
#[command(about = "task to bench", long_about = None)]
struct Args {
    #[arg(short, long)]
    task: String,

    #[arg(short, long)] // default: False
    prove_only: bool,
}
pub fn concat(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut c = Vec::with_capacity(a.len() + b.len());
    c.extend_from_slice(a);
    c.extend_from_slice(b);
    c
}

pub fn hash_elem(input: u8) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(&[input]);
    hasher.finalize().to_vec()
}

fn main() {
    sp1_sdk::utils::setup_logger();
    let args = Args::parse();
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    let leaf = hash_elem(12);

    // Convert and hash the path integers
    let path: Vec<Vec<u8>> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .into_iter()
        .map(|x| hash_elem(x as u8))
        .collect();

    // Calculate the expected root
    let mut hasher = Keccak256::new();
    let concat_first = concat(&leaf, &path[0]);
    hasher.update(&concat_first);
    let mut current_hash = hasher.finalize_reset().to_vec();

    for path_element in path.iter().skip(1) {
        let concat_next = concat(&current_hash, path_element);
        hasher.update(&concat_next);
        current_hash = hasher.finalize_reset().to_vec();
    }

    let expected_root = current_hash;

    stdin.write(&leaf);
    stdin.write(&path);
    stdin.write(&expected_root);

    let mut execution_time = 0.0;
    let mut num_cycles = 0;
    let mut num_mem_access = 0;
    if !args.prove_only {
        let start = Instant::now();
        let (_output, report) = client.execute(ELF, stdin.clone()).run().unwrap();
        execution_time = start.elapsed().as_secs_f64();
        println!("program execution: ✅");
        num_cycles = report.total_instruction_count();
        num_mem_access = report.touched_memory_addresses;
    }
    // Proving the program
    let start = Instant::now();
    let (pk, vk) = client.setup(ELF);
    let setup_time = start.elapsed().as_secs_f64();
    println!("key generation: ✅");

    let start = Instant::now();
    let mut proof = client.prove(&pk, stdin).run().unwrap();
    let prover_time = start.elapsed().as_secs_f64();
    println!("proof generation: ✅");

    if args.prove_only {
        exit(0);
    }
    let result = proof.public_values.read::<i32>();

    // Verify proof and public values
    let start = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    let verifier_time = start.elapsed().as_secs_f64();
    println!("proof verification: ✅");
    println!("Merkle Verification: {:?}", result == 1);
    // Test a round trip of proof serialization and deserialization.
    proof
        .save(args.task.to_owned() + "_proof-with-pis.bin")
        .expect("saving proof failed");

    let proof_file = File::open(args.task.to_owned() + "_proof-with-pis.bin").unwrap();
    let metadata = proof_file.metadata().unwrap();
    let proof_size = metadata.len();
    println!("File size: {} bytes", proof_size);
    let deserialized_proof =
        SP1ProofWithPublicValues::load(args.task.to_owned() + "_proof-with-pis.bin")
            .expect("loading proof failed");

    // Verify the deserialized proof.
    client
        .verify(&deserialized_proof, &vk)
        .expect("verification failed");

    let results = BenchResults {
        framework: "sp1".to_string(),
        task: args.task.clone(),
        execution_time,
        setup_time,
        prover_time,
        verifier_time,
        proof_size,
        num_cycles,
        num_mem_access,
        speed: (num_cycles as f64) / prover_time,
    };

    let json = to_string_pretty(&results).unwrap();
    let mut file = File::create(args.task.to_owned() + ".json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
