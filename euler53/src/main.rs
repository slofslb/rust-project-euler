extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    // 先把一些阶乘计算好，保存起来
    let mut fact = vec![BigUint::from(1 as u64); 101];
    let mut a = BigUint::from(1 as u64);
    for n in 2..=100 {
        a *= BigUint::from(n as u64);
        fact[n] = a.clone();
    }
    //println!("{:?}", fact);

    let mut count = 0;
    for n in 1..=100 {
        for r in 1..=n {
            let comb = &fact[n] / &fact[r] / &fact[n - r];
            if comb > BigUint::from(1_000_000 as u64) {
                println!("{} {} {}", n, r, comb.to_string());
                // count += 1;
                // 把上面count += 1换成下面两行，可以大幅优化性能
                // 比如C(90, 1), C(90, 2), C(90, 3)都小于1百万
                // 在C(90, 4)时，值大于1百万
                // 根据组合数的性质，C(90, 86) 一定也肯定大于1百万
                // 这样不用进行大量的计算，可以知道大于1百万的组合有 86-4+1 = 83组
                count += n - r - r + 1;
                break;
            }
        }
    }
    println!("{}", count);
}
// 4075
