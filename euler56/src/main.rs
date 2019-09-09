extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut max_sum = 0;
    for a in (1..100).rev() {
        for b in (1..100).rev() {
            let s = power(a, b).to_string();
            let sum_digits = s.chars().map(|ch| ch.to_digit(10).unwrap()).sum::<u32>();
            if sum_digits > max_sum {
                max_sum = sum_digits;
                println!(
                    "a: {} b: {} len: {} sum of digits: {}",
                    a,
                    b,
                    s.len(),
                    sum_digits
                );
            }
        }
    }
    println!("{}", max_sum);
}

fn power(a: u64, b: u64) -> BigUint {
    let mut prod = BigUint::from(a as u64);
    for _i in 0..b {
        prod *= BigUint::from(a as u64);
    }
    prod
}
