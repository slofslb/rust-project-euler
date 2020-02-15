use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let f = BufReader::new(File::open("matrix.txt").unwrap());
    let arr: Vec<Vec<usize>> = f
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    //println!("{:?}", arr);

    let min_path = compute_min_path(&arr);
    println!("{}", min_path[0][0]);
}
// 427337

fn compute_min_path(w: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let cols = w[0].len();
    let rows = w.len();
    let mut path: Vec<Vec<usize>> = vec![vec![0; cols]; rows];
    for i in (0..rows).rev() {
        for j in (0..cols).rev() {
            if i == rows - 1 && j == cols - 1 {
                path[i][j] = w[i][j];
            } else {
                let mut down = std::usize::MAX;
                if i + 1 < rows {
                    down = path[i + 1][j];
                }
                let mut right = std::usize::MAX;
                if j + 1 < cols {
                    right = path[i][j + 1];
                };
                path[i][j] = w[i][j] + down.min(right);
            }
        }
    }
    path
}
