extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut v: Vec<BigUint> = vec![];
    for a in 2_u64..=100 {
        let mut prod = BigUint::from(a);
        for _b in 2_u64..=100 {
            prod *= BigUint::from(a);
            if !v.contains(&prod) {
                v.push(prod.clone());
            }
        }
    }
    println!("{}", v.len());
}
// 9183 Debug下运行10多秒，release模式1秒

fn power(a: u64, b: u64) -> BigUint {
    let mut prod = BigUint::from(1 as u64);
    for _i in 0..b {
        prod *= BigUint::from(a);
    }
    prod
}

/*
let s = full_str
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum::<u32>();
println!("{}", s);
*/
