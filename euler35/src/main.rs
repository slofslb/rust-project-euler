fn main() {
    let mut count_primes = 0;
    for n in 2..1_000_000 {
        if is_rotate_prime(n) {
            println!("{}", n);
            count_primes += 1;
        }
    }
    println!("{}", count_primes);

    // another style
    let count_primes = (2..1_000_000).filter(|&x| is_rotate_prime(x)).count();
    println!("{}", count_primes);
}

fn is_rotate_prime(n: u64) -> bool {
    if n.to_string().contains('0') {
        return false;
    }
    let mut r = n;
    for _i in 0..n.to_string().len() {
        if !primes::is_prime(r) {
            return false;
        }
        r = rotate(r);
    }
    true
}

fn rotate(n: u64) -> u64 {
    let mut s = n.to_string();
    let ch = s.remove(0);
    s.push(ch);
    s.parse::<u64>().unwrap()
}

// version 1
fn rotate_v1(n: u64) -> u64 {
    let mut s = n.to_string();
    let ch = s.chars().next().unwrap();
    s = s[1..].to_string();
    s.push(ch);
    s.parse::<u64>().unwrap()
}
