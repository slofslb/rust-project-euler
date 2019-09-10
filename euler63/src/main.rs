extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut count = 0;
    for b in 1..100 {
        for a in 1..=9 {
            let p = power(a, b).to_string();
            if p.len() == b as usize {
                count += 1;
                println!("{} {} {} {}", count, a, b, p);
            }
        }
    }
    println!("{}", count);
}

fn power(a: u64, b: u64) -> BigUint {
    let mut p = BigUint::from(a);
    for _i in 1..b {
        p *= BigUint::from(a);
    }
    p
}
