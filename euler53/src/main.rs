extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    // 先把一些阶乘计算好，保存起来
    let mut fact = vec![BigUint::from(1 as u64); 101];
    let mut a = BigUint::from(1 as u64);
    for n in 2..=100 {
        a *= BigUint::from(n as u64);
        fact[n] = a.clone();
    }
    println!("{:?}", fact);

    let mut count = 0;
    for n in 1..=100 {
        for r in 1..=n {
            let comb = &fact[n] / &fact[r] / &fact[n - r];
            if comb > BigUint::from(1_000_000 as u64) {
                println!("{} {} {}", n, r, comb.to_string());
                count += 1;
            }
        }
    }
    println!("{}", count);
}
// 4075
