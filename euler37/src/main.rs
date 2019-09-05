fn main() {
    let max_number_to_check = 1_000_000;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    let mut sum = 0;
    for n in 10..max_number_to_check {
        if is_trun_prime_right(&prime_mask, n) && is_trun_prime_left(&prime_mask, n) {
            println!("{}", n);
            sum += n;
        }
    }
    println!("sum: {}", sum);
}

fn is_trun_prime_right(prime_mask: &Vec<bool>, n: usize) -> bool {
    let mut s = n.to_string();
    while s.len() > 0 {
        let p = s.parse::<usize>().unwrap();
        if !prime_mask[p] {
            return false;
        }
        s.remove(0);
    }
    true
}

fn is_trun_prime_left(prime_mask: &Vec<bool>, n: usize) -> bool {
    let mut s = n.to_string();
    while s.len() > 0 {
        let p = s.parse::<usize>().unwrap();
        if !prime_mask[p] {
            return false;
        }
        s.pop();
    }
    true
}

fn fill_prime_mask(prime_mask: &mut Vec<bool>) {
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
