
pub fn prime_sieve(max_number_to_check:usize) -> Vec<bool> {
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
    prime_mask
}