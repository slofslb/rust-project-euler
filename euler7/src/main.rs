fn main() {
    let max_number_to_check = 1_000_000;

    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;

    let mut total_primes_found = 0;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            // println!("{}", p);
            total_primes_found += 1;
            if total_primes_found == 10001 {
                println!("the 10001st prime number is : {}", p);
                break;
            }
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }

    // use primes module
    use primes::{Sieve, PrimeSet};
    let mut pset = Sieve::new();
    println!("{}", pset.get(10_000));
}
// 104743