use clap::Parser;
use methods::{MERKLE_ELF, MERKLE_ID};
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv};
use serde_json::to_string_pretty;
use sha3::{Digest, Keccak256};
use std::{fs::File, io::Write, process::exit, time::Instant};
#[derive(serde::Serialize)]
struct BenchResults {
    framework: String,
    task: String,
    execution_time: f64,
    prover_time: f64,
    verifier_time: f64,
    proof_size: u64,
    num_cycles: u64,
    speed: f64,
}

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
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();

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

    let input = (leaf, path, expected_root);

    let env_exec = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();
    let env_prove = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // execution
    let mut execution_time = 0.0;
    if !args.prove_only {
        let exec = default_executor();
        let start = Instant::now();
        let _session = exec.execute(env_exec, MERKLE_ELF).unwrap();
        execution_time = start.elapsed().as_secs_f64();
        println!("program execution: ✅");
    }
    // proving.
    let prover = default_prover();
    let start = Instant::now();
    let prove_info = prover.prove(env_prove, MERKLE_ELF).unwrap();
    let prover_time = start.elapsed().as_secs_f64();
    let num_cycles = prove_info.stats.total_cycles;
    let speed = num_cycles as f64 / prover_time;
    println!("proof generation: ✅");
    if args.prove_only {
        exit(0);
    }
    // extract the receipt.
    let receipt = prove_info.receipt;
    // from https://github.com/succinctlabs/zkvm-perf/blob/main/eval/src/risc0.rs
    let composite_receipt = receipt.inner.composite().unwrap();
    let num_segments = composite_receipt.segments.len();

    // Get the core proof size by summing across all segments.
    let mut proof_size = 0 as u64;
    for segment in composite_receipt.segments.iter() {
        proof_size += segment.seal.len() as u64 * 4;
    }

    let start = Instant::now();
    receipt.verify(MERKLE_ID).unwrap();
    let verifier_time = start.elapsed().as_secs_f64();
    println!("proof verification: ✅");
    let result: i32 = receipt.journal.decode().unwrap();

    println!("Merkle Verification Result: {}", result == 1);

    let results = BenchResults {
        framework: "risc0".to_string(),
        task: args.task.clone(),
        execution_time,
        prover_time,
        verifier_time,
        proof_size,
        num_cycles,
        speed,
    };

    let json = to_string_pretty(&results).unwrap();
    let mut file = File::create(args.task.to_owned() + ".json").unwrap();
    file.write_all(json.as_bytes()).unwrap()
}
