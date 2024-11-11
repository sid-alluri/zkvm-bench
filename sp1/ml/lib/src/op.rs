use crate::{helper, processing::Tensor, shape::*};
pub fn conv2d(
    inp: Vec<Vec<Vec<i32>>>,
    kernel: Tensor,
    bias: Tensor,
    stride: i32,
    padding: i32,
    reluoff: bool,
) -> Vec<Vec<Vec<i32>>> {
    let mut image = inp;
    let kernel = helper::kernel_reshape(kernel);
    let bias = helper::bias_reshape(bias);
    if padding == 0 {
        let padlen = (kernel[0][0].len() - 1) / 2;
        println!("padlen: {}", padlen);
        let mut padded_image = vec![];
        for i in 0..image.len() + 2 * padlen {
            let mut row = vec![];
            for j in 0..image[0].len() + 2 * padlen {
                let mut col = vec![];
                for k in 0..image[0][0].len() {
                    if i < padlen
                        || j < padlen
                        || i >= image.len() + padlen
                        || j >= image[0].len() + padlen
                    {
                        col.push(0);
                    } else {
                        col.push(image[i - padlen][j - padlen][k]);
                    }
                }
                row.push(col);
            }
            padded_image.push(row);
        }
        image = padded_image;
    }
    //shape of image
    println!(
        "image shape: {}, {}, {}",
        image.len(),
        image[0].len(),
        image[0][0].len()
    );
    // println!("image: {:?}", image);
    let mut output_dim: Vec<i32> = vec![0, 0, 0];
    output_dim[0] = (image.len() as i32 - kernel[0].len() as i32) / stride + 1;
    output_dim[1] = (image[0].len() as i32 - kernel[0][0].len() as i32) / stride + 1;
    output_dim[2] = kernel.len() as i32;

    let mut out = vec![];
    for i in 0..output_dim[0] as usize {
        out.push(vec![]);
        for j in 0..output_dim[1] as usize {
            out[i].push(vec![]);
            for k in 0..output_dim[2] as usize {
                out[i][j].push(0);
                for n in 0..kernel[0][0][0].len() as usize {
                    for m in 0..kernel[0].len() as usize {
                        for l in 0..kernel[0][0].len() as usize {
                            let buf = image[i * stride as usize + m][j * stride as usize + l][n]
                                * kernel[k][m][l][n];
                            out[i][j][k] += buf;
                        }
                    }
                }
                out[i][j][k] /= 512;
                out[i][j][k] += bias[k];
                if !reluoff {
                    out[i][j][k] = relu6(out[i][j][k]);
                }
            }
        }
    }
    //shape of out
    println!(
        "out shape: {}, {}, {}",
        out.len(),
        out[0].len(),
        out[0][0].len()
    );
    return out;
}

pub fn relu6(inp: i32) -> i32 {
    if inp < 0 {
        return 0;
    } else if inp > 6 * 512 {
        return 6 * 512;
    } else {
        return inp;
    }
}

pub fn avgpool2d(inp: Vec<Vec<Vec<i32>>>, params: Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let mut output_dim: Vec<i32> = vec![0, 0, 0];
    output_dim[0] = (inp.len() as i32 - params[0]) / params[2] + 1;
    output_dim[1] = (inp[0].len() as i32 - params[1]) / params[3] + 1;
    output_dim[2] = inp[0][0].len() as i32;
    let mut out = vec![];
    for i in 0..output_dim[0] as usize {
        out.push(vec![]);
        for j in 0..output_dim[1] as usize {
            out[i].push(vec![]);
            for k in 0..output_dim[2] as usize {
                out[i][j].push(0);
                for m in 0..params[0] as usize {
                    for l in 0..params[1] as usize {
                        out[i][j][k] +=
                            inp[i * params[2] as usize + m][j * params[3] as usize + l][k];
                    }
                }
                let div_factor = (params[0] * params[2]) as i32;
                out[i][j][k] /= div_factor;
            }
        }
    }
    return out;
}

pub fn add(inp: Vec<Vec<Vec<i32>>>, bias: Tensor) -> Vec<Vec<Vec<i32>>> {
    let mut out = inp.clone();
    let bias = helper::bias_reshape(bias);
    for i in 0..inp.len() {
        for j in 0..inp[0].len() {
            for k in 0..inp[0][0].len() {
                out[i][j][k] += bias[k];
            }
        }
    }
    return out;
}

pub fn mul(inp: Vec<Vec<Vec<i32>>>, scale: Tensor) -> Vec<Vec<Vec<i32>>> {
    let mut out = inp.clone();
    let scale = helper::bias_reshape(scale);
    println!("scale {:?}", scale);
    for i in 0..inp.len() {
        for j in 0..inp[0].len() {
            for k in 0..inp[0][0].len() {
                out[i][j][k] *= scale[k];
                out[i][j][k] /= 512;
            }
        }
    }
    return out;
}
pub fn relu(inp: i32) -> i32 {
    if inp < 0 {
        return 0;
    } else {
        return inp;
    }
}

pub fn fc_layer(
    inp: Vec<Vec<i32>>,
    kernel: Vec<Vec<i32>>,
    bias: Vec<i32>,
    params: Vec<i32>,
    biasoff: bool,
) -> Vec<Vec<i32>> {
    let mut out = vec![];
    let transposed_kernel = transpose(kernel);
    for i in 0..inp.len() {
        out.push(vec![]);
        for j in 0..transposed_kernel[0].len() {
            out[i].push(0);
            for k in 0..inp[0].len() {
                let buf = inp[i][k] * transposed_kernel[k][j];
                out[i][j] += buf;
            }
            out[i][j] /= 512;
            if !biasoff {
                out[i][j] += bias[j];
            }
            if params[0] == 1 {
                out[i][j] = relu(out[i][j]);
            }
        }
    }
    return out;
}
