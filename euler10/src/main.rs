use primes::PrimeSet;

fn main() {
    let max_number_to_check = 2_000_000;

    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;

    let mut sum: u64 = 0;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            sum += p as u64;
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }
    println!("{}", sum);

    // 另一种写法
    let mut pset = PrimeSet::new();
    println!("{}", (2..2_000_000).filter(|x| pset.is_prime(*x)).sum::<u64>());
    // 或者：
    println!("{}", (2..2_000_000).filter(|x| primes::is_prime(*x)).sum::<u64>());
}
// 142913828922
