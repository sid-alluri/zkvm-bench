use clap::Parser;
use methods::{ML_ELF, ML_ID};
use mllib::processing;
use risc0_zkvm::{default_executor, default_prover, ExecutorEnv};
use serde_json::to_string_pretty;
use std::{fs::File, io::Write, process::exit, time::Instant};
#[derive(serde::Serialize)]
struct BenchResults {
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

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // input and env
    let args = Args::parse();
    let (env_exec, env_prove) = get_env(args.task.clone());

    // execution
    let mut execution_time = 0.0;
    if !args.prove_only {
        let exec = default_executor();
        let start = Instant::now();
        let _session = exec.execute(env_exec, ML_ELF).unwrap();
        execution_time = start.elapsed().as_secs_f64();
        println!("program execution: ✅");
    }
    // proving.
    let prover = default_prover();
    let start = Instant::now();
    let prove_info = prover.prove(env_prove, ML_ELF).unwrap();
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
    let _output: Vec<Vec<Vec<i32>>> = receipt.journal.decode().unwrap();
    let start = Instant::now();
    receipt.verify(ML_ID).unwrap();
    let verifier_time = start.elapsed().as_secs_f64();
    println!("proof verification: ✅");
    println!("{:?}", _output);

    let results = BenchResults {
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

fn get_env(task: String) -> (ExecutorEnv<'static>, ExecutorEnv<'static>) {
    if task == "mnist" {
        let (_shape, input_data) = processing::load_data("mnist_inp.json".to_string());
        let model_data = processing::load_model("converted_mnist_model_sf512.json".to_string());
        let input = (task, model_data, input_data);
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
        return (env_exec, env_prove);
    } else {
        let input_data = gen_dlrm_data();
        let model_data = processing::load_model("converted_dlrm_checked_512.json".to_string());
        let input = (task, model_data, input_data);
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
        return (env_exec, env_prove);
    }
}

pub fn gen_dlrm_data() -> Vec<i32> {
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
    return merge_data(dense, sparse);
}

pub fn merge_data(dense: Vec<i32>, sparse: Vec<Vec<i32>>) -> Vec<i32> {
    let mut merged = dense.clone();
    for vec in sparse {
        merged.extend(vec);
    }
    merged
}
