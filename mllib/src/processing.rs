use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub global_sf: i32,
    pub k: i32,
    pub num_cols: i32,
    pub num_random: i32,
    pub inp_idxes: Vec<i32>,
    pub out_idxes: Vec<i32>,
    pub layers: Vec<Layer>,
    pub tensors: Vec<Tensor>,
    pub use_selectors: bool,
    pub commit_before: Vec<i32>,
    pub commit_after: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layer {
    pub layer_type: String,
    pub inp_idxes: Vec<i32>,
    pub inp_shapes: Vec<Vec<i32>>,
    pub out_idxes: Vec<i32>,
    pub out_shapes: Vec<Vec<i32>>,
    pub params: Vec<i32>,
    pub mask: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tensor {
    pub idx: i32,
    pub shape: Vec<i32>,
    pub data: Vec<i32>,
}

pub fn load_model(path: String) -> ModelConfig {
    let file = File::open(&Path::new(&path)).expect("Failed to open file");
    let reader = BufReader::new(file);
    let model_data: ModelConfig = serde_json::from_reader(reader).expect("Failed to parse JSON");
    return model_data;
}

pub fn load_data(path: String) -> (Vec<i32>, Vec<i32>) {
    let file = File::open(&Path::new(&path)).expect("Failed to open file");
    let reader = BufReader::new(file);
    let json_data: Value = serde_json::from_reader(reader).expect("Failed to parse JSON");
    let data_arr = json_data.as_array().unwrap();
    let shape: Vec<f64> = data_arr[0]["shape"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(|value| value.as_f64())
        .collect();
    let data: Vec<f64> = data_arr[0]["data"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter_map(|value| value.as_f64())
        .collect();
    let shape: Vec<i32> = shape.iter().map(|&x| x as i32).collect();
    let data: Vec<i32> = data.iter().map(|&x| x as i32).collect();
    return (shape, data);
}
