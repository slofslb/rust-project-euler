fn main() {
    let max_number_to_check = 1000_000;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    let mut len = 1;
    for start in 2..1000_000 {
        if !prime_mask[start] {
            continue;
        }
        let mut count = 1;
        let mut sum = start;
        for i in start + 1..1000_000 {
            if prime_mask[i] {
                count += 1;
                sum += i;
                if sum > 1_000_000 {
                    break;
                }
                if count > len && prime_mask[sum] {
                    len = count;
                    println!("start: {} len: {} sum: {}", start, len, sum);
                }
            }
        }
    }
}

fn fill_prime_mask(prime_mask: &mut [bool]) {
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
