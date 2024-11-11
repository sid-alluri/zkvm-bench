use crate::helper;
use crate::helper::*;
use crate::op::*;
use crate::processing::*;

pub fn mnist(model: &ModelConfig, data: &Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let mut output = vec![vec![vec![]]];
    output = helper::inp_reshape(&data, vec![28, 28, 1]);
    for i in 0..model.layers.len() {
        let layer = &model.layers[i];
        if layer.layer_type == "Conv2D" && layer.params[0] == 0 {
            let kernel = get_tensor(layer.inp_idxes[1], &model.tensors);
            let bias = get_tensor(layer.inp_idxes[2], &model.tensors);
            let stride = layer.params[3];
            let padding = layer.params[1];
            let mut reluoff = false;
            if layer.params[2] == 0 {
                reluoff = true;
            }
            output = conv2d(output, kernel, bias, stride, padding, reluoff);
        } else if layer.layer_type == "AveragePool2D" {
            output = avgpool2d(output, layer.clone().params)
        } else if layer.layer_type == "Mul" {
            let scale = get_tensor(layer.inp_idxes[1], &model.tensors);
            output = mul(output, scale);
        } else if layer.layer_type == "Add" {
            let bias = get_tensor(layer.inp_idxes[1], &model.tensors);
            output = add(output, bias);
        } else {
            println!("Layer {} not supported", layer.layer_type);
        }
    }
    return output;
}
