fn main() {
    for n in (3..).step_by(2) {
        if primes::is_prime(n) {
            continue;
        } // 奇合数
        if !can_divide(n) {
            println!("{}", n);
            break;
        }
    }
}

fn can_divide(n: u64) -> bool {
    let limit = ((n / 2) as f64).sqrt() as u64;
    for i in 1..=limit {
        let p = n - 2 * i * i;
        if primes::is_prime(p) {
            return true;
        }
    }
    false
}
