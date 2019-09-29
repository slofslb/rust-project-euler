fn main() {
    let mut count = 0;
    let mut sum = 0;
    for n in 10.. {
        if is_trun_left_prime(n) && is_trun_right_prime(n) {
            println!("{}", n);
            count += 1;
            sum += n;
            if count == 11 {break;}
        }
    }
    println!("sum: {}", sum);
}

// 左截素数
fn is_trun_left_prime(n: u64) -> bool {
    let mut s = n.to_string();
    while s.len() > 0 {
        let p = s.parse::<u64>().unwrap();
        if !primes::is_prime(p) {
            return false;
        }
        s.remove(0);
    }
    true
}

// 右截素数
fn is_trun_right_prime(n: u64) -> bool {
    let mut s = n.to_string();
    while s.len() > 0 {
        let p = s.parse::<u64>().unwrap();
        if !primes::is_prime(p) {
            return false;
        }
        s.pop();
    }
    true
}
