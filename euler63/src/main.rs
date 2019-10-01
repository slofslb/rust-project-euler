extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut count = 0;
    for a in 1..=9 {
        for b in 1.. {
            let p = power(a, b).to_string();
            //println!("{} ^ {} = {} {}", a, b, p, p.len());
            if p.len() < b as usize {
                // 位数永远不可能超过幂次了，退出内层循环
                break;
            }
            if p.len() == b as usize {
                count += 1;
                println!("{}:  {} ^ {} = {}", count, a, b, p);
            }
        }
    }
    println!("{}", count);
}
// 49

fn power(a: u64, b: u64) -> BigUint {
    let mut p = BigUint::from(a);
    for _i in 1..b {
        p *= BigUint::from(a);
    }
    p
}
