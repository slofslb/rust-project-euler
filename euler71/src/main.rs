fn main() {
    let mut max = 0.0;
    let mut numerator = 0;
    for d in 2..=1_000_000 { // 分母
        let n = d * 3 / 7;
        if n * 7 == d * 3 {
            continue;
        }
        let q = (n as f64) / (d as f64);
        if q > max {
            max = q;
            numerator = n;
            //println!("{} / {} = {}", numerator, denom, q);
        }
    }
    println!("{}", numerator);
}
