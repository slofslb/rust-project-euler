fn main() {
    // 借鉴第7题的代码
    let mut prime_mask = vec![true; 2_000_000];
    sieve_prime_mask(&mut prime_mask);

    let mut max_prime_len = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            let prime_series_len = consecutive_primes(&prime_mask, a, b);
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

// 用筛选法生成素数
fn sieve_prime_mask(prime_mask: &mut [bool]) {
    prime_mask[0] = false;
    prime_mask[1] = false;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..prime_mask.len() {
        if prime_mask[p] {
            let mut i = 2 * p;
            while i < prime_mask.len() {
                prime_mask[i] = false;
                i += p;
            }
        }
    }
}

// 公式可以生成多少个连续的素数
fn consecutive_primes(prime_mask: &[bool], a: isize, b: isize) -> u32 {
    for n in 0..1000 {
        // 这里求值时，可能会出现负数，如果用usize，运行时会出现溢出错误
        let y: isize = n * n + a * n + b;
        if y < 0 || !prime_mask[y as usize] {
            return n as u32;
        }
    }
    0
}
