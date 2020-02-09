fn main() {
    let mat = vec![
        7, 53, 183, 439, 863, // row 0
        497, 383, 563, 79, 973, // row 1
        287, 63, 343, 169, 583, // row 2
        627, 343, 773, 959, 943, // row 3
        767, 473, 103, 699, 303, // row 4
    ];
    let mut cache = vec![0; 2_usize.pow(5)];
    let mut path = vec![];
    assert_eq!(3315, matrix_sum(&mut cache, &mat, &mut path));

    let mat = vec![
        7, 53, 183, 439, 863, 497, 383, 563, 79, 973, 287, 63, 343, 169, 583, 627, 343, 773, 959,
        943, 767, 473, 103, 699, 303, 957, 703, 583, 639, 913, 447, 283, 463, 29, 23, 487, 463,
        993, 119, 883, 327, 493, 423, 159, 743, 217, 623, 3, 399, 853, 407, 103, 983, 89, 463, 290,
        516, 212, 462, 350, 960, 376, 682, 962, 300, 780, 486, 502, 912, 800, 250, 346, 172, 812,
        350, 870, 456, 192, 162, 593, 473, 915, 45, 989, 873, 823, 965, 425, 329, 803, 973, 965,
        905, 919, 133, 673, 665, 235, 509, 613, 673, 815, 165, 992, 326, 322, 148, 972, 962, 286,
        255, 941, 541, 265, 323, 925, 281, 601, 95, 973, 445, 721, 11, 525, 473, 65, 511, 164, 138,
        672, 18, 428, 154, 448, 848, 414, 456, 310, 312, 798, 104, 566, 520, 302, 248, 694, 976,
        430, 392, 198, 184, 829, 373, 181, 631, 101, 969, 613, 840, 740, 778, 458, 284, 760, 390,
        821, 461, 843, 513, 17, 901, 711, 993, 293, 157, 274, 94, 192, 156, 574, 34, 124, 4, 878,
        450, 476, 712, 914, 838, 669, 875, 299, 823, 329, 699, 815, 559, 813, 459, 522, 788, 168,
        586, 966, 232, 308, 833, 251, 631, 107, 813, 883, 451, 509, 615, 77, 281, 613, 459, 205,
        380, 274, 302, 35, 805,
    ];

    // 算法一：遗传算法 
    // 绝大部分时候能够得到正确结果
    let reproduce_num = 7;
    let mut paths = vec![vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]];
    let mut max = 0;
    for _generation in 0..20000 {
        for _i in 0..reproduce_num * reproduce_num {
            let path = &paths[rand::random::<usize>() % paths.len()];
            let path_new = swap(
                &path,
                rand::random::<usize>() % 15,
                rand::random::<usize>() % 15,
            );
            if !paths.contains(&path_new) {
                let ev = eval(&mat, &path_new);
                if ev > max {
                    max = ev;
                    paths.insert(0, path_new);
                } else {
                    paths.push(path_new);
                }
            }
        }
        // 只保留前几个优质的
        if paths.len() > reproduce_num {
            for p in (reproduce_num..paths.len()).rev() {
                paths.remove(p);
            }
        }
    }
    println!("{} {:?}", max, paths[0]);

    // 算法二：递归算法 
    let mut cache = vec![0; 2_usize.pow(15)];
    let mut path = vec![];
    println!("{}", matrix_sum(&mut cache, &mat, &mut path));
    matrix_output(&cache, &mat);
}
// 13938
// [5, 14, 7, 4, 3, 11, 10, 2, 13, 0, 1, 9, 12, 6, 8]

fn swap(path: &[usize], i: usize, j: usize) -> Vec<usize> {
    let mut path2 = path.to_owned();
    path2[i] = path[j];
    path2[j] = path[i];
    path2
}

fn eval(mat: &[usize], path: &[usize]) -> usize {
    let dim = (mat.len() as f64).sqrt() as usize;
    let mut sum = 0;
    for (col, row) in path.iter().enumerate() {
        let index = row * dim + col;
        sum += mat[index];
    }
    sum
}

fn matrix_sum(cache: &mut [usize], mat: &[usize], path: &mut Vec<usize>) -> usize {
    // 总列数
    let num_cols = (mat.len() as f64).sqrt() as usize;
    // 准备填充的列号
    let cur_col = path.len();
    if cur_col == num_cols {
        return 0;
    }

    let hash = hash_code(path);
    if cache[hash] > 0 {
        return cache[hash];
    }

    let mut max_sum = 0;
    for row in 0..num_cols {
        if !path.contains(&row) {
            path.push(row);
            let temp = mat[row * num_cols + cur_col] + matrix_sum(cache, &mat, path);
            if temp > max_sum {
                max_sum = temp;
            }
            path.pop();
        }
    }
    cache[hash] = max_sum;
    //println!("{:?} {} {}", &path, hash, max_sum);
    max_sum
}

fn hash_code(path: &[usize]) -> usize {
    path.iter().map(|p| 1_usize << p).sum()
    /* 
    let mut hash = 0;
    for p in path {
        hash += 1_usize << p;
    }
    hash
    */
}

// ugly codes to output the path!
fn matrix_output(cache: &[usize], mat: &[usize]) {
    let num_cols = (mat.len() as f64).sqrt() as usize;
    let mut path = vec![];
    for col in 0..num_cols {
        let mut selected_row = 999;
        let mut max = 0;
        for row in 0..num_cols {
            if !path.contains(&row) {
                path.push(row);
                let hash = hash_code(&path);
                if mat[row * num_cols + col] + cache[hash] > max {
                    max = mat[row * num_cols + col] + cache[hash];
                    selected_row = row;
                }
                path.pop();
            }
        }
        path.push(selected_row);
    }
    println!("path: {:?}", path);
}


/* version 0.1 递归算法，但运行非常慢
fn main() {
    let mat = vec![
        7, 53, 183, 439, 863, // row 0
        497, 383, 563, 79, 973, // row 1
        287, 63, 343, 169, 583, // row 2
        627, 343, 773, 959, 943, // row 3
        767, 473, 103, 699, 303, // row 4
    ];
    println!("{}", matrix_sum(&mat));
}

fn matrix_sum(mat: &Vec<usize>) -> usize {
    if mat.len() == 1 {
        return mat[0];
    }

    let dim = (mat.len() as f64).sqrt() as usize;
    let mut max_sum = 0;
    for row in 0..dim {
        let m2 = residual_matrix(&mat, row, 0);
        let temp = mat[row*dim] + matrix_sum(&m2);
        if temp > max_sum {
            max_sum = temp;
        }
    }
    max_sum
}

// 剩余矩阵
fn residual_matrix(mat: &Vec<usize>, row:usize, col:usize) -> Vec<usize> {
    let dim = (mat.len() as f64).sqrt() as usize;
    let mut m2 = vec![];
    for index in 0..mat.len() {
        let r = index / dim;
        let c = index % dim;
        if row != r && col != c {
            m2.push(mat[index]);
        }
    }
    m2
}
*/