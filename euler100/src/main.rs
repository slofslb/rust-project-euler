use std::time::SystemTime;

fn main() {
    /*
    let start = SystemTime::now();
    let sq = 0.5_f64.sqrt();
    for n in 1000_000_000_000_u64.. {
        let b = (sq * (n as f64) + (1.0 - sq)) as u64;
        if n * (n-1) == 2 * b * (b - 1) {
            println!("n:{} b:{}", n, b);
            break;
        }
    }
    println!("{:?}", start.elapsed());
    */

    println!("------------------------");
    let sqrt_1_2 = 0.5_f64.sqrt();
    let start = SystemTime::now();
    let mut n = 21_u64;
    loop {
        let b = (sqrt_1_2 * (n as f64) + (1.0 - sqrt_1_2)) as u64;
        if n * (n - 1) == 2 * b * (b - 1) {
            println!("n:{} b:{}", n, b);
            if n > 1000_000_000_000_u64 {
                break;
            }
            n = (n as f64 * 5.8284) as u64;
        }
        n += 1;
    }
    println!("{:?}", start.elapsed());

    main1();
}
// n:1070379110497 b:756872327473

fn main1() {
    for n in 2..100_000_u64 {
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
