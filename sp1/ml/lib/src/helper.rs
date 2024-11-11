use crate::processing::Tensor;
pub fn get_tensor(tensor_idx: i32, tensors: &Vec<Tensor>) -> Tensor {
    let mut req_tensor = Tensor {
        idx: 0,
        shape: vec![],
        data: vec![],
    };
    for i in 0..tensors.len() {
        if tensors[i].idx == tensor_idx {
            req_tensor = tensors[i].clone();
        }
    }
    return req_tensor;
}

pub fn inp_reshape(data: &Vec<i32>, shape: Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let mut reshaped_data: Vec<Vec<Vec<i32>>> = vec![];
    let mut idx = 0;
    for _i in 0..shape[0] as usize {
        let mut row: Vec<Vec<i32>> = vec![];
        for _j in 0..shape[1] as usize {
            let mut col: Vec<i32> = vec![];
            for _k in 0..shape[2] as usize {
                col.push(data[idx]);
                idx += 1;
            }
            row.push(col);
        }
        reshaped_data.push(row);
    }
    return reshaped_data;
}

pub fn kernel_reshape(kernel: Tensor) -> Vec<Vec<Vec<Vec<i32>>>> {
    let mut reshaped_kernel = vec![];
    let shape = kernel.shape;
    let data = kernel.data;
    let mut idx = 0;
    for _i in 0..shape[0] as usize {
        let mut row = vec![];
        for _j in 0..shape[1] as usize {
            let mut col = vec![];
            for _k in 0..shape[2] as usize {
                let mut dep = vec![];
                for _l in 0..shape[3] as usize {
                    dep.push(data[idx as usize] as i32);
                    idx += 1;
                }
                col.push(dep);
            }
            row.push(col);
        }
        reshaped_kernel.push(row);
    }
    return reshaped_kernel;
}

pub fn bias_reshape(bias: Tensor) -> Vec<i32> {
    let shape = bias.shape;
    let data = bias.data;
    let mut reshaped_bias = vec![];
    for i in 0..shape[0] as usize {
        reshaped_bias.push(data[i] as i32);
    }
    return reshaped_bias;
}

pub fn fc_kernel_reshape(kernel: Tensor) -> Vec<Vec<i32>> {
    let shape = kernel.shape;
    let data = kernel.data;
    let mut reshaped_kernel = vec![];
    let mut idx = 0;
    for _i in 0..shape[0] as usize {
        let mut row = vec![];
        for _j in 0..shape[1] as usize {
            row.push(data[idx] as i32);
            idx += 1;
        }
        reshaped_kernel.push(row);
    }
    return reshaped_kernel;
}

pub fn get_gather_idx() -> Vec<i32> {
    let gather_idx = vec![
        27, 54, 55, 81, 82, 83, 108, 109, 110, 111, 135, 136, 137, 138, 139, 162, 163, 164, 165,
        166, 167, 189, 190, 191, 192, 193, 194, 195, 216, 217, 218, 219, 220, 221, 222, 223, 243,
        244, 245, 246, 247, 248, 249, 250, 251, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279,
        297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 324, 325, 326, 327, 328, 329, 330,
        331, 332, 333, 334, 335, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363,
        378, 379, 380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 405, 406, 407, 408,
        409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419, 432, 433, 434, 435, 436, 437, 438,
        439, 440, 441, 442, 443, 444, 445, 446, 447, 459, 460, 461, 462, 463, 464, 465, 466, 467,
        468, 469, 470, 471, 472, 473, 474, 475, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495,
        496, 497, 498, 499, 500, 501, 502, 503, 513, 514, 515, 516, 517, 518, 519, 520, 521, 522,
        523, 524, 525, 526, 527, 528, 529, 530, 531, 540, 541, 542, 543, 544, 545, 546, 547, 548,
        549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559, 567, 568, 569, 570, 571, 572, 573,
        574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 594, 595, 596, 597,
        598, 599, 600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614, 615,
        621, 622, 623, 624, 625, 626, 627, 628, 629, 630, 631, 632, 633, 634, 635, 636, 637, 638,
        639, 640, 641, 642, 643, 648, 649, 650, 651, 652, 653, 654, 655, 656, 657, 658, 659, 660,
        661, 662, 663, 664, 665, 666, 667, 668, 669, 670, 671, 675, 676, 677, 678, 679, 680, 681,
        682, 683, 684, 685, 686, 687, 688, 689, 690, 691, 692, 693, 694, 695, 696, 697, 698, 699,
        702, 703, 704, 705, 706, 707, 708, 709, 710, 711, 712, 713, 714, 715, 716, 717, 718, 719,
        720, 721, 722, 723, 724, 725, 726, 727,
    ];
    return gather_idx;
}
