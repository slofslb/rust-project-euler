extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    let mut count = 0;
    let mut a = BigUint::from(3 as u64);
    let mut b = BigUint::from(2 as u64);
    for _i in 2..=10 {
        let c = &a + &b;
        a = &c + &b;
        b = c;
            println!("{} {}", a, b);
        if a.to_string().len() > b.to_string().len() {
            count += 1;
        }
    }
    println!("{}", count);
}
// 153
