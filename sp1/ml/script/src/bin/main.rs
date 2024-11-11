//ref: https://github.com/succinctlabs/sp1/blob/dev/examples/fibonacci/script/src/main.rs
// ref: https://github.com/succinctlabs/zkvm-perf/blob/main/eval/src/sp1.rs
use clap::Parser;
use mllib::processing; // TODO: name change from fib_lib
use serde_json::to_string_pretty;
use sp1_sdk::{ProverClient, SP1ProofWithPublicValues, SP1Stdin};
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::Instant;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");
#[derive(serde::Serialize)]
struct BenchResults {
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
fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();
    if args.task == "mnist" {
        mnist(&mut stdin);
    } else if args.task == "dlrm" {
        dlrm(&mut stdin);
    }

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

    let output = proof.public_values.read::<Vec<Vec<Vec<i32>>>>();
    println!("output: {:?}", output);

    if args.prove_only {
        exit(0);
    }

    // Verify proof and public values
    let start = Instant::now();
    client.verify(&proof, &vk).expect("verification failed");
    let verifier_time = start.elapsed().as_secs_f64();
    println!("proof verification: ✅");

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
    file.write_all(json.as_bytes()).unwrap()
}

pub fn mnist(stdin: &mut SP1Stdin) {
    // Setup the inputs.
    let task = 0_u8; //mnist
    stdin.write(&task);

    let (shape, data) = processing::load_data("../mnist_inp.json".to_string());
    stdin.write(&shape);
    stdin.write(&data);

    let model_data = processing::load_model("../converted_mnist_model_sf512.json".to_string());
    stdin.write(&model_data);
}

pub fn dlrm(stdin: &mut SP1Stdin) {
    let task = 1_u8; //dlrm
    stdin.write(&task);

    let (dense, sparse) = gen_dlrm_data();
    stdin.write(&dense);
    stdin.write(&sparse);

    // Give model weights
    let model_data = processing::load_model("../converted_dlrm_checked_512.json".to_string());
    stdin.write(&model_data);
}

pub fn gen_dlrm_data() -> (Vec<i32>, Vec<Vec<i32>>) {
    let mut dense: Vec<i32> = vec![]; // [26*64] all 0s
    let mut sparse: Vec<Vec<i32>> = vec![]; // [1,13] all 0s

    for _i in 0..26 * 64 {
        dense.push(0);
    }
    for _i in 0..1 {
        let mut temp: Vec<i32> = vec![];
        for _j in 0..13 {
            temp.push(0);
        }
        sparse.push(temp);
    }
    (dense, sparse)
}
