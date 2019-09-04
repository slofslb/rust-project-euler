extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut prod = BigUint::from(1 as u64);
    for i in [2;1000].iter() {
        prod *= BigUint::from(*i as u64);
    }

    // 另一种写法
    let pow2_1000 = [2;1000].iter().fold(BigUint::from(1 as u64), |p, a| p*BigUint::from(*a as u64));
    println!("{}", pow2_1000);

    let full_str = prod.to_str_radix(10);

    let sum = full_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0, |s, a| s + a);

    println!("{}", full_str);
    println!("{}", sum);
}
