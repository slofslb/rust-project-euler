extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut prev = BigUint::from(1 as u64);
    let mut cur = BigUint::from(1 as u64);
    for i in 3.. {
        let next = prev + &cur;
        let str = next.to_string();
        if str.len() >= 1000 {
            println!("{} {} {}", i, str, str.len());
            break;
        }
        // println!("{} {} {}", i, str, str.len());
        prev = cur;
        cur = next;
    }
}
// 4782