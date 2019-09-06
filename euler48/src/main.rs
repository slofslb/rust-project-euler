extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut sum: BigUint = BigUint::from(0 as u64);
    for a in 1..=1000 {
        sum += power(a, a);
    }
    println!("{}", sum);
}

fn power(a: u64, b: u64) -> BigUint {
    let mut prod = BigUint::from(1 as u64);
    for _i in 0..b {
        prod *= BigUint::from(a);
    }
    prod
}
