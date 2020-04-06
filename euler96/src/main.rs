use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let f = BufReader::new(File::open("p096_sudoku.txt").unwrap());
    for line in f.lines().map(|l| l.unwrap()) {
        println!("{:?}", line);
    }
}