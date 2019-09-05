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

    let mut count_primes = 0;
    for n in 2..max_number_to_check {
        let len = n.to_string().len();
        let mut count = 0;
        let mut r = n;
//        if !prime_mask[n] {continue;}
        for _i in 0..len {
            if prime_mask[r] {
                count += 1;
            }
            else {
                break;
            }
            r = rotate(r);
            if r.to_string().len() != len {break;} //位数减少，说明遇到了0
        }
        if count == len {
            println!("{}", n);
            count_primes += 1;
        }
    }
    println!("{}", count_primes);
}

fn rotate(n: usize) -> usize {
    let mut s = n.to_string();
    let ch = s.remove(0);
    s.push(ch);
    s.parse::<usize>().unwrap()
}

// version 1
fn rotate_v1(n:u32) -> u32 {
    let mut s = n.to_string();
    let ch = s.chars().next().unwrap();
    s = s[1..].to_string();
    s.push(ch);
    s.parse::<i32>().unwrap() as u32
}