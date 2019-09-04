fn main() {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - a - b;
            if c > 0 && a*a + b*b == c*c {
                println!("{} = {} x {} x {}", a*b*c, a, b, c);
                return;
            }
        }
    }
}
