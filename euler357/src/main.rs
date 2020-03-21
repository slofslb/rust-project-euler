mod myprime;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    let large_num = 100_000_000;
    // 为防止判断 d + n / d 是否为素数时越界，多分配一点空间
    let prime_mask = myprime::prime_sieve(large_num + 2); 
    println!("finished prime generation");

    let mut sum = 0;
    for n in 1..=large_num {
        if is_divisor_prime(&prime_mask, n) {
            sum += n;
        }
    }
    println!("{:?}", start.elapsed());
    println!("{}", sum);
}
// 1739023853137

fn is_divisor_prime(prime_mask: &[bool], n: usize) -> bool {
    let s = (n as f32).sqrt() as usize;

    // 把一半的因子全求出来，计算量太大，性能非常差
    // let half_divisors: Vec<usize> = (1..=s).filter(|x| n % x == 0).collect();

    // 这个闭包特性非常有用，并没有完全计算出来，而是延迟评价
    let half_divisors = (1..=s).filter(|x| n % x == 0);

    for d in half_divisors {
        let p = d + n / d;
        if !prime_mask[p] {
            return false;
        }
    }
    true
}

// 优化过程：
// 1. 只求一半因子
// 2. 闭包，不是立刻求值
