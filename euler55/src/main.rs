extern crate num_bigint;
use num_bigint::BigUint;

// 需要这个use语句，还不明白原因
use std::str::FromStr;

fn main() {
    let mut count = 0;
    for n in 1..10000 {
        if is_lychrel_number(n) {
            println!("{}", n);
            count += 1;
        }
    }
    println!("count: {}", count);
}

fn is_lychrel_number(n: u64) -> bool {
    let mut x = BigUint::from(n);
    for _i in 1..=50 {
        x = lychrel_transform(&x);
        if is_palindromic(&x) {
            return false;
        }
    }
    true
}

fn lychrel_transform(n: &BigUint) -> BigUint {
    let rev_str = n.to_string().chars().rev().collect::<String>();
    let rev_n = BigUint::from_str(&rev_str).unwrap();
    n + rev_n
}

fn is_palindromic(n: &BigUint) -> bool {
    let str_n = n.to_string();
    let rev_str = str_n.chars().rev().collect::<String>();
    str_n == rev_str
}
// 249

/* first version  u64 will overflow
fn main() {
    for n in 1..10000 {
        if is_lychrel_number(n) {
            println!("{}", n);
        }
    }
}

fn is_lychrel_number(n: u64) -> bool {
    let mut x = n;
    for _i in 1..=50 {
        x = lychrel_transform(x);
        if is_palindromic(x) {
            return false;
        }
    }
    true
}

fn lychrel_transform(n: u64) -> u64 {
    let rev_str = n.to_string().chars().rev().collect::<String>();
    let rev_n = rev_str.parse::<u64>().unwrap();
    n + rev_n
}

fn is_palindromic(n: u64) -> bool {
    let rev_str = n.to_string().chars().rev().collect::<String>();
    n.to_string() == rev_str
}
*/
