fn main() {
    let mut count = 0;
    for d in 2..=8 { 
        for n in 1..d {
            let r = (n as f64) / (d as f64);
            if  1.0 / 3.0 < r && r < 1.0 / 2.0 {
                println!("{} / {}", n, d);
                count += 1;
            }
        }
    }
    println!("{}", count);
}
