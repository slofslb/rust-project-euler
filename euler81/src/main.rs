fn main() {
    let data = std::fs::read_to_string("matrix.txt").expect("读文件失败");
    let data2 = data.trim().replace("\n", ",");
    let w: Vec<usize> = data2
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    //println!("{:?}", w);
    let path = compute_path_weight(&w);
    println!("{}", path[0]);
}

fn compute_path_weight(w: &Vec<usize>) -> Vec<usize> {
    let mut path: Vec<usize> = vec![0; w.len()];
    for i in (0..=79 + 79).rev() {
        for r in (0..80).rev() {
            if i < r {
                continue;
            }
            let c = i - r;
            if c >= 80 {
                continue;
            }
            let node = r * 80 + c;
            if r == 79 && c == 79 {
                path[node] = w[node];
            } else {
                let down = if r >= 79 {
                    std::usize::MAX
                } else {
                    w[node] + path[node + 80]
                };
                let right = if c >= 79 {
                    std::usize::MAX
                } else {
                    w[node] + path[node + 1]
                };
                path[node] = if down < right { down } else { right };
            }
        }
    }
    return path;
}

// 读文件并初始化二维数组的办法
/*
let mut matrix = vec![vec![0; 80]; 80];
let data = std::fs::read_to_string("matrix.txt").expect("读文件失败");
let lines = data.trim().split('\n');
for (i,line) in lines.enumerate() {
    let lines_data = line.split(',')
        .map(|x| x.parse::<usize>().unwrap());
    for (j, weight) in lines_data.enumerate() {
        matrix[i][j] = weight;
    }
}
*/

/*
    let mut w = vec![std::usize::MAX; 160 * 81];
    let data = std::fs::read_to_string("matrix.txt").expect("读文件失败");
    let lines = data.trim().split('\n');
    for (i,line) in lines.enumerate() {
        let lines_data = line.split(',')
            .map(|x| x.parse::<usize>().unwrap());
        for (j, weight) in lines_data.enumerate() {
            w[i*160 + j] = weight;
        }
    }
*/
