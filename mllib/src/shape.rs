use std::vec;
pub fn transpose(inp: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    for i in 0..inp[0].len() {
        out.push(vec![]);
        for j in 0..inp.len() {
            out[i].push(inp[j][i]);
        }
    }
    return out;
}

pub fn concatenate(input1: Vec<Vec<i32>>, input2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    for i in 0..input1.len() {
        out.push(vec![]);
        for j in 0..input1[0].len() {
            out[i].push(input1[i][j]);
        }
        for j in 0..input2[0].len() {
            out[i].push(input2[i][j]);
        }
    }
    return out;
}

pub fn reshape1dto2d(input: Vec<Vec<i32>>, shape: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let mut idx = 0;
    for _i in 0..shape[0] as usize {
        out.push(vec![]);
        for _j in 0..shape[1] as usize {
            out[_i].push(input[0][idx]);
            idx += 1;
        }
    }
    return out;
}

pub fn reshape2dto1d(input: Vec<Vec<i32>>, _shape: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let mut idx = 0;
    for i in 0..input.len() as usize {
        for j in 0..input[0].len() as usize {
            out.push(vec![]);
            out[idx].push(input[i][j]);
            idx += 1;
        }
    }
    return out;
}
