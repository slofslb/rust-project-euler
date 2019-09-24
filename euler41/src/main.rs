// 借鉴第24题，再改为itertools的实现
use itertools::Itertools;

fn main() {
    let it = (1..=7).combinations(7);
    let mut max_prime = 0;
    for v in it {
        // vec![1, 2, 3, 4, 5, 6, 7] -> 1234567
        println!("{:?}", v);
        let d = v.iter().fold(0, |x, a| 10 * x + a);
        if primes::is_prime(d as u64) && d > max_prime {
            println!("{}", d);
            max_prime = d;
        }
    }
}
// 7652413