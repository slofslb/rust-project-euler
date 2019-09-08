extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut v: Vec<BigUint> = vec![];
    for a in 2..=100 {
        for b in 2..=100 {
            let x = power(a, b);
            if !v.contains(&x) {
                v.push(x);
            }
        }
    }
    println!("{:?}", v.len());
}
// 9183 运行10多秒

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
