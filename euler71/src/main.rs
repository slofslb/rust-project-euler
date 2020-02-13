fn main() {
    println!("3/7= {}", 3.0 / 7.0);
    let mut max = 0.0;
    for numerator in 2..=1_000_000 {
        let x = numerator * 3 / 7;
        if x * 7 == numerator * 3 {
            continue;
        }
        let q = (x as f64) / (numerator as f64);
        if q > max {
            max = q;
            println!("{} / {} = {}", x, numerator, q);
        }
    }
}
