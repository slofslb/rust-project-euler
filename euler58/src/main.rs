// 大量素数的判断时，用PrimeSet，比primes::is_prime()要快
use primes::PrimeSet;

fn main() {
    let mut pset = PrimeSet::new();

    let mut count_prime = 0;
    for n in (3..).step_by(2) {
        let lower_right = n * n;
        let prime_four_corner = (0..4)
            .map(|x| lower_right - (n - 1) * x)
            .filter(|&x| pset.is_prime(x));
        count_prime += prime_four_corner.count();

        let percent = (count_prime as f32) / ((2 * n - 1) as f32);
        if percent < 0.1 {
            println!("{} count: {} percent: {}", n, count_prime, percent);
            break;
        }
    }
}
// 26241 count: 5248 percent: 0.099998094
