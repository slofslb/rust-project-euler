use primes::PrimeSet;

fn main() {
    let mut pset = PrimeSet::new();
    let mut vp = vec![];
    for p in pset.iter() {
        if p > 1_000_000 {break;}
        vp.push(p);
    }

    let limit = 1_000_000;
    // 记录连续素数的长度
    let mut prime_len = 1;
    for start in 2..=limit {
        if !primes::is_prime(start) {
            continue;
        }
        let mut count = 1;
        let mut sum = start;
        for i in start + 1..=limit {
            if primes::is_prime(i) {
                count += 1;
                sum += i;
                if sum >= limit {
                    break;
                }
                if count > prime_len && primes::is_prime(sum) {
                    prime_len = count;
                    println!("start: {} consecutive primes len: {} sum: {}", start, prime_len, sum);
                }
            }
        }
    }
}
// 997651

