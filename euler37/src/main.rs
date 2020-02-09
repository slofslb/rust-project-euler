fn main() {
    let mut count = 0;
    let mut sum = 0;
    for n in 10.. {
        if is_trunc_left_prime(n) && is_trunc_right_prime(n) {
            println!("{}", n);
            count += 1;
            sum += n;
            if count == 11 {break;}
        }
    }
    println!("sum: {}", sum);

    // 另一种写法
    println!("{}", (10..).filter(|&n| is_trunc_left_prime(n) && is_trunc_right_prime(n))
    .take(11).sum::<u64>());
}
// 748317

// 左截素数
fn is_trunc_left_prime(n: u64) -> bool {
    let mut s = n.to_string();
    while !s.is_empty() {
        let p = s.parse::<u64>().unwrap();
        if !primes::is_prime(p) {
            return false;
        }
        s.remove(0);
    }
    true
}

// 右截素数
fn is_trunc_right_prime(n: u64) -> bool {
    let mut m = n;
    while m > 0 {
        if !primes::is_prime(m) {
            return false;
        }
        m /= 10;
    }
    true
}
