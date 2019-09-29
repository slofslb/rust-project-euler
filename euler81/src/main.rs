fn main() {
    // 读到一个一维数组中
    let data = std::fs::read_to_string("matrix.txt").expect("读文件失败");
    let data2 = data.trim().replace("\n", ",");
    let w: Vec<usize> = data2
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    //println!("{:?}", w); 
    let min_path = compute_min_path(&w);
    println!("{}", min_path[0]);
}

fn compute_min_path(w: &Vec<usize>) -> Vec<usize> {
    let mut path: Vec<usize> = vec![0; w.len()];
    for &node in node_by_layer().iter().rev() {
        let r = node / 80;
        let c = node % 80;
        // the lowest and rightest node
        if r == 79 && c == 79 {
            path[node] = w[node];
        } else {
            let down = if r >= 79 {
                std::usize::MAX
            } else {
            // 一维数组存放，下一行的节点编号就是node + 80
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
    return path;
}

// 按斜线这种顺序存储节点
// 0  1  3  6  10
// 2  4  7  11
// 5  8  12
// 9  13
// 14
fn node_by_layer() -> Vec<usize> {
    let mut nodes = vec![];
    for layer in 0..=79 + 79 {
        for row in 0..80 {
            if layer < row {
                continue;
            }
            let col = layer - row;
            if col >= 80 {
                continue;
            }
            let node_id = row * 80 + col;
            nodes.push(node_id);
            if row == 79 && col == 79 {
                break;
            }
        }
    }
    nodes
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
