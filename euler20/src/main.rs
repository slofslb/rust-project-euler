extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    // 与16题非常相似
    let mut prod = BigUint::from(1 as u64);
    for i in 1..=100 {
        prod *= BigUint::from(i as u64);
    }

    let full_str = prod.to_str_radix(10);
    println!("{}", full_str);

    let s = full_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    println!("{}", s);
}
