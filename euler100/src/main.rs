fn main() {
    let sqrt_1_2 = 0.5_f64.sqrt();
    for n in 2..10_000_000 {
        let b = (sqrt_1_2 * (n as f64) + (1.0 - sqrt_1_2)) as u64;
        if n * (n - 1) == 2 * b * (b - 1) {
            println!("n:{} b:{}", n, b);
        }
    }

    // main1();
}


fn main1() {
    for n in 2..200_000_u64 {
        let temp = n * (n-1) / 2;
        for b in 2..n {
            if temp == b * (b - 1) {
                println!("n:{} b:{}", n, b);
                break;
            }
        }
    }
}
// n:4 b:3
// n:21 b:15
// n:120 b:85
// n:697 b:493
// n:4060 b:2871
// n:23661 b:16731
