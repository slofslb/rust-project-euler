use primes::PrimeSet;
mod myprime;

fn main() {
    let prime_masks = myprime::prime_sieve(2_000_000);

    let mut sum = 0;
    for (i, &mask) in prime_masks.iter().enumerate() {
        if mask {
            sum += i as u64;
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
