// ugly, but can work!
fn main() {
    let mut count = 0;
    for a in 0..=200 {
        for b in 0..=100 {
            for c in 0..=40 {
                for d in 0..=20 {
                    for e in 0..=10 {
                        for f in 0..=4 {
                            for g in 0..=2 {
                                for h in 0..=1 {
                                    let price = a
                                        + b * 2
                                        + c * 5
                                        + d * 10
                                        + e * 20
                                        + f * 50
                                        + g * 100
                                        + h * 200;
                                    if price == 200 {
                                        count += 1;
                                        //println!("{}", count);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", count);
}
