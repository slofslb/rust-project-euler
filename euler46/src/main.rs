fn main() {
    let max_number_to_check = 10000;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    for n in (3..).step_by(2) {
        if prime_mask[n] {continue;} // 奇合数
        if !can_divide(&prime_mask, n) {
            println!("{}", n);
            break;
        }
    }
}

fn can_divide(prime_mask: &[bool], n: usize) -> bool {
    let limit = ((n / 2) as f64).sqrt() as usize;
    for i in 1..=limit {
        let p = n - 2 * i * i;
        if prime_mask[p] {
            return true;
        }
    }
    false
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
