fn main() {
    let sum_of_squares = (1..=100).fold(0, |s, n| s + n * n);
    let sum = (1..=100).fold(0, |s, n| s + n);
    println!("{}", sum * sum - sum_of_squares);
}
// 25164150
