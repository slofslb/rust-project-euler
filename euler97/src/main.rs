fn main() {
    let mut p = 28433_u64;
    for _i in 0..7830457 {
        p = p * 2 % 10_000_000_000;
    }
    println!("{}", p + 1);

    let mersenne = [2; 7830457]
        .iter()
        .fold(28433_u64, |s, _| s * 2 % 10_000_000_000)
        + 1;
    println!("{}", mersenne);
}
