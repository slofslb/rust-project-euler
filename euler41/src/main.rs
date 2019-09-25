// 借鉴第24题，再改为依赖第三方的排列组合库来实现
use permutohedron::heap_recursive;

fn main() {
    let mut max_prime = 0;
    let mut data = [1, 2, 3, 4, 5, 6, 7];
    heap_recursive(&mut data, |permutation| {
        let v = permutation.to_vec();
        // println!("{:?}", v);
        // vec![1, 2, 3, 4, 5, 6, 7] -> 1234567
        let d = v.iter().fold(0, |x, a| 10 * x + a);
        if primes::is_prime(d as u64) && d > max_prime {
            println!("{}", d);
            max_prime = d;
        }
    })
}
// 7652413