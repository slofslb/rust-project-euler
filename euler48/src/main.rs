
fn main() {
    let mut sum: u64 = 0;
    for a in 1..=1000 {
        sum = (sum + power_last_10(a, a)) % 10_000_000_000;
    }
    println!("{}", sum);
}

fn power_last_10(a: u64, b: u64) -> u64 {
    let mut prod = 1;
    for _i in 0..b {
        prod = prod * a % 10_000_000_000;
    }
    prod
}

// 第二种办法，利用大整数函数库
extern crate num_bigint;
use num_bigint::BigUint;

fn main2() {
    let mut sum: BigUint = BigUint::from(0 as u64);
    for a in 1..=1000 {
        sum += power(a, a);
    }
    let str_sum = sum.to_string();
    println!("{}", &str_sum[str_sum.len()-10..]);

}

fn power(a: u64, b: u64) -> BigUint {
    let mut prod = BigUint::from(1 as u64);
    for _i in 0..b {
        prod *= BigUint::from(a);
    }
    prod
}
