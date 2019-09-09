use primes::PrimeSet;

fn main() {
    let mut pset = PrimeSet::new();

    let mut count = 0;
    for n in (3..).step_by(2) {
        let lower_right = n * n;

        let diag = [lower_right, lower_right - (n-1), lower_right - (n-1)*2, lower_right - (n-1)*3];
        for d in diag.iter() {
            if pset.is_prime(*d) {
                count += 1;
            }
        }
        let percent = (count as f32) / ((2 * n - 1) as f32);
        if percent < 0.1 {
            println!("{} count: {} percent: {}", n, count, percent);
            break;
        }
    }
}
