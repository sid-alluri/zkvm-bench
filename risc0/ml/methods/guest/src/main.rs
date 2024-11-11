use mllib::dlrm;
use mllib::mnist;
use mllib::processing::ModelConfig;
use risc0_zkvm::guest::env;
fn main() {
    // TODO: Implement your guest code here
    // read the input
    let (task, model_data, input_data): (String, ModelConfig, Vec<i32>) = env::read();
    if task == "mnist" {
        let output = mnist(model_data, input_data);
        env::commit(&output);
    } else if task == "dlrm" {
        let output = dlrm(model_data, input_data);
        env::commit(&output);
    }
}

fn mnist(model_data: ModelConfig, input_data: Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let output = mnist::mnist(&model_data, &input_data);
    return output;
}

fn dlrm(model_data: ModelConfig, input_data: Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let (dense, sparse) = demerge_data(input_data);
    let output = dlrm::dlrm(&model_data, &dense, &sparse);
    return vec![output];
}

pub fn demerge_data(merged: Vec<i32>) -> (Vec<i32>, Vec<Vec<i32>>) {
    let dense_size = 26 * 64;
    let sparse_size = 13;

    let dense = merged[0..dense_size].to_vec();
    let sparse_data = merged[dense_size..].to_vec();

    let mut sparse = vec![];
    sparse.push(sparse_data[0..sparse_size].to_vec());

    (dense, sparse)
}
