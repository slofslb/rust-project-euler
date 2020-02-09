fn main() {
    println!("{}", ways(200, 0));
}

fn ways(money: isize, maxcoin: usize) -> usize {
    let coins = [200, 100, 50, 20, 10, 5, 2, 1];
    let mut sum = 0;
    if maxcoin == 7 {
        return 1;
    }
    for (i, coin) in coins.iter().enumerate().skip(maxcoin) {
        if money - coin == 0 {
            sum += 1;
        }
        if money - coin > 0 {
            sum += ways(money - coin, i);
        }
    }
    sum
}
// 73682

// ugly, but can work!
/*
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
*/
