use crate::helper::*;
use crate::op::*;
use crate::processing::*;
use crate::shape::*;

pub fn dlrm(model: &ModelConfig, dense: &Vec<i32>, sparse: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut output = sparse.clone();
    println!("sparse shape: {}, {}", output.len(), output[0].len());
    let reshaped_dense = inp_reshape(&dense, vec![26, 1, 64]); // [26*64] -> [26,1,64]

    for i in 0..3 {
        let layer = &model.layers[i];
        if layer.layer_type == "FullyConnected" {
            let kernel = get_tensor(layer.inp_idxes[1], &model.tensors);
            let kernel = fc_kernel_reshape(kernel);
            let bias: Tensor = get_tensor(layer.inp_idxes[2], &model.tensors);
            let bias = bias_reshape(bias);
            output = fc_layer(output, kernel, bias, layer.clone().params, false);
        }
    }
    let output_copy = output.clone();
    for i in 0..reshaped_dense.len() {
        output = concatenate(output, reshaped_dense[i].clone());
    }
    let reshaped_output = reshape1dto2d(output, vec![27, 64]);
    output = fc_layer(
        reshaped_output.clone(),
        reshaped_output.clone(),
        vec![0],
        vec![0],
        true,
    );
    let reshaped_output_2 = reshape2dto1d(output, vec![729, 1]);
    // gather
    let gather_idx = get_gather_idx();
    let mut gathered_output = vec![];
    for i in 0..gather_idx.len() {
        gathered_output.push(vec![]);
        gathered_output[i].push(reshaped_output_2[gather_idx[i] as usize][0].clone());
    }
    output = transpose(gathered_output);
    output = concatenate(output_copy, output);
    println!("output shape: {}, {}", output.len(), output[0].len());
    for i in 10..14 {
        let layer = &model.layers[i];
        if layer.layer_type == "FullyConnected" {
            let kernel = get_tensor(layer.inp_idxes[1], &model.tensors);
            let kernel = fc_kernel_reshape(kernel);
            let bias: Tensor = get_tensor(layer.inp_idxes[2], &model.tensors);
            let bias = bias_reshape(bias);
            output = fc_layer(output, kernel, bias, layer.clone().params, false);
        }
    }
    return output;
}
