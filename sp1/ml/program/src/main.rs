//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use mllib::dlrm;
use mllib::mnist;
use mllib::processing;
use mllib::processing::ModelConfig;

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
    return (dense, sparse);
}

// for testing, comment no main
pub fn execute_dummy_mnist() {
    let (_shape, data) = processing::load_data("../mnist_inp.json".to_string());

    // Give model weights
    let model_data = processing::load_model("../converted_mnist_model_sf512.json".to_string());
    let output = mnist::mnist(&model_data, &data);
    println!("output: {:?}", output);
}

pub fn execute_mnist() {
    let _inp_shape = sp1_zkvm::io::read::<Vec<i32>>();
    let inp_data = sp1_zkvm::io::read::<Vec<i32>>();
    let model_data = sp1_zkvm::io::read::<ModelConfig>();
    let output = mnist::mnist(&model_data, &inp_data);
    println!("output: {:?}", output);
    sp1_zkvm::io::commit(&output);
}

pub fn execute_dummy_dlrm() {
    let (dense, sparse) = gen_dlrm_data();
    let model_data = processing::load_model("../converted_dlrm_checked_512.json".to_string());
    let output = dlrm::dlrm(&model_data, &dense, &sparse);
    println!("output: {:?}", output);
}

pub fn execute_dlrm() {
    let inp_dense = sp1_zkvm::io::read::<Vec<i32>>(); // [26*64]
    let inp_sparse = sp1_zkvm::io::read::<Vec<Vec<i32>>>(); // [1,13]
    let model_data = sp1_zkvm::io::read::<ModelConfig>();
    let output = dlrm::dlrm(&model_data, &inp_dense, &inp_sparse);
    println!("output: {:?}", output);
    sp1_zkvm::io::commit(&vec![output]);
}

pub fn main() {
    let task = sp1_zkvm::io::read::<u8>();
    match task {
        0 => execute_mnist(),
        1 => execute_dlrm(),
        _ => panic!("Invalid task"),
    }
}

// use fibonacci_lib::{fibonacci, PublicValuesStruct};

// pub fn main() {
//     // Read an input to the program.
//     //
//     // Behind the scenes, this compiles down to a custom system call which handles reading inputs
//     // from the prover.
//     let n = sp1_zkvm::io::read::<u32>();

//     // Compute the n'th fibonacci number using a function from the workspace lib crate.
//     let (a, b) = fibonacci(n);

//     // Encode the public values of the program.
//     let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct { n, a, b });

//     // Commit to the public values of the program. The final proof will have a commitment to all the
//     // bytes that were committed to.
//     sp1_zkvm::io::commit_slice(&bytes);
// }
