use primes::PrimeSet;

fn main() {
    let mut pset = PrimeSet::new();

    let mut count = 0;
    for n in (3..).step_by(2) {
        let lower_right = n * n;
        if pset.is_prime(lower_right) {
            count += 1;
        }
        let lower_left = lower_right - n + 1;
        if pset.is_prime(lower_left) {
            count += 1;
        }
        let upper_left = lower_left - n + 1;
        if pset.is_prime(upper_left) {
            count += 1;
        }
        let upper_right = upper_left - n + 1;
        if pset.is_prime(upper_right) {
            count += 1;
        }
        let percent = (count as f32) / ((2 * n - 1) as f32);
        if percent < 0.1 {
            println!("{} {} {}", n, count, percent);
            break;
        }
    }
}
