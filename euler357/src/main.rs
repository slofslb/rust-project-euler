fn main() {
    let max_number_to_check = 101_000_000;
    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;
    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }
    println!("finished prime generation");

    let mut sum = 0;
    for n in 1..100_000_000 {
        if n % 10_000_000 == 0 {
            println!("{}", n);
        }
        if is_divisor_prime(&prime_mask, n) {
            sum += n;
        }
    }
    println!("{}", sum);
}
// 1739023853137

fn is_divisor_prime(prime_mask: &[bool], n: usize) -> bool {
    let s = (n as f32).sqrt() as usize;
    // 这个闭包特性非常有用，并没有完全计算出来，而是延迟评价
    let half_divisors = (1..=s).filter(|x| n % x == 0);

    for d in half_divisors {
        let p = d + n / d;
        if !prime_mask[p] {
            return false;
        }
        //println!("{}", p);
    }
    true
}

// 把一半的因子全求出来
fn half_divisors(num: usize) -> Vec<usize> {
    let s = (num as f32).sqrt() as usize;
    (1..=s).filter(|x| num % x == 0).collect::<Vec<usize>>()
}

// 优化过程：
// 1. 只求一半因子
// 2. 闭包，不是立刻求值
