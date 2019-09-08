fn main() {
    // 借鉴第7题的代码
    let max_number_to_check = 2_000_000;

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

    let mut max_prime_len = 0;
    for a in -999..=999 {
        for b in -1000..=1000 {
            let mut prime_series_len = 0;
            for n in 0..1000 {
                //println!("n*n - {}*n + {}, n: {}", a, b, n);
                // 这里求值时，可能会出现负数，如果用usize，运行时会出现溢出错误
                let y: isize = n * n + a * n + b;
                if y < 0 || !prime_mask[y as usize] {
                    prime_series_len = n;
                    break;
                }
            }
            if prime_series_len > max_prime_len {
                max_prime_len = prime_series_len;
                println!(
                    "primes: {} a: {} + b: {}   a * b = {}",
                    prime_series_len,
                    a,
                    b,
                    a * b
                );
            }
        }
    }
}
