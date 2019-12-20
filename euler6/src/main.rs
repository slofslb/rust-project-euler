fn main() {
    let sum_of_squares = (1..=100).map(|x| x*x).sum::<u32>();
    let square_sum = (1..=100).sum::<u32>().pow(2);
    println!("{}", square_sum - sum_of_squares);

    // 另一种写法
    let sum_of_squares = (1..=100).fold(0, |s, n| s + n * n);
    let square_sum = (1..=100_u64).fold(0, |s, n| s + n).pow(2);
    println!("{}", square_sum - sum_of_squares);
}
// 25164150
