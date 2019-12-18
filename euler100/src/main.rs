fn main() {
    for n in 1..100_000 {
        for b in 2..n {
            if n * (n - 1) == 2 * b * (b - 1) {
                println!("n:{} b:{}", n, b);
                break;
            }
        }
    }
}
