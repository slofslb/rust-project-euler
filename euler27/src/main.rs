fn main() {
    let mut max_prime_len = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            let prime_series_len = consecutive_primes(a, b);
            if prime_series_len > max_prime_len {
                max_prime_len = prime_series_len;
                println!(
                    "primes: {} a: {} b: {}   a * b = {}",
                    prime_series_len,
                    a,
                    b,
                    a * b
                );
            }
        }
    }
}
// -59231

// 公式可以生成多少个连续的素数
fn consecutive_primes(a: i64, b: i64) -> u64 {
    for n in 0..1000 {
        // 这里求值时，可能会出现负数，如果用usize，运行时会出现溢出错误
        let y: i64 = n * n + a * n + b;
        if y < 0 || !primes::is_prime(y as u64) {
            return n as u64;
        }
    }
    0
}
