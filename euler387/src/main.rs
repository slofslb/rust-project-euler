fn main() {
    let max_number_to_check = 1_000_000;
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
    println!("finished prime generation");

    println!("{}", harshad(201));
    println!("{}", right_harshad(201));
    println!("{}", strong_right_harshad(&prime_mask, 201));
}

fn harshad(n: u64) -> bool {
    let sum_digits: u32 = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum();
    n % (sum_digits as u64) == 0
}

fn right_harshad(n: u64) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num = 0;
    let mut sum = 0;
    for d in digits {
        num = num * 10 + d;
        sum += d;
        if num % sum != 0 {
            return false;
        }
    }
    true
}

fn strong_right_harshad(prime_mask: &Vec<bool>, n: u64) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num = 0;
    let mut sum = 0;
    for d in digits {
        num = num * 10 + d;
        sum += d;
        if num % sum != 0 {
            return false;
        }
    }
    prime_mask[(n as usize) / (sum as usize)]
}
