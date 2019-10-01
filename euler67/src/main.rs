// 第18题的强化版
fn main() {
    let data = std::fs::read_to_string("triangle.txt").expect("读文件失败");
    let data2 = data.trim().replace("\r\n", " ").replace("\n", " ");
    let w: Vec<usize> = data2
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let path = compute_path_weight(&w);
    println!("{:?}", path[0]);
}
// 7273

fn compute_path_weight(w: &Vec<usize>) -> Vec<usize> {
    let mut path: Vec<usize> = vec![0; w.len()];
    let max_row: usize = row(w.len() - 1);
    for i in (0..w.len()).rev() { // 从底层向上计算
        let r = row(i);
        if r == max_row { // leaf
            path[i] = w[i];
        } else {
            let left = w[i] + path[i + r];
            let right = w[i] + path[i + r + 1];
            path[i] = if left > right { left } else { right };
        }
    }
    return path;
}

fn row(n: usize) -> usize {
    let mut s = 0;
    for r in 1.. {
        s += r;
        if s > n {
            return r;
        }
    }
    return 0;
}
