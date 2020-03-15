use primes::PrimeSet;

fn main() {
    /* // find an error answer: 129976621
    let mut pset = PrimeSet::new();
    for p in pset.iter() {
        if is_concat_primes(p, 3) && is_concat_primes(3, p)
            && is_concat_primes(p, 7) && is_concat_primes(7, p)
            && is_concat_primes(p, 109) && is_concat_primes(109, p)
            && is_concat_primes(p, 673) && is_concat_primes(673, p)
        {
            println!("{}", p);
            break;
        }
    }
    */

    let max = 10000;
    for p1 in prime_pairs(max, &[]) {
        for p2 in prime_pairs(max, &p1) {
            for p3 in prime_pairs(max, &p2) {
                for p4 in prime_pairs(max, &p3) {
                    let p5 = prime_pairs(max, &p4);
                    if !p5.is_empty() {
                        println!("{:?}", p5[0]);
                        println!("{:?}", p5[0].iter().sum::<u64>());
                    }
                }
            }
        }
    }
}

// right: 26033 = [13, 5197, 5701, 6733, 8389]

// try to append a prime to list_primes, don't exceed max value
fn prime_pairs(max: u64, list_primes: &[u64]) -> Vec<Vec<u64>> {
    let mut pairs = vec![];

    let mut pset = PrimeSet::new();
    for p in pset.iter().take_while(|&x| x <= max) {
        if list_primes.is_empty() {
            pairs.push(vec![p]);
            continue;
        }
        if p > *list_primes.last().unwrap() && all_primes(list_primes, p) {
            let mut v = list_primes.to_vec();
            v.push(p);
            pairs.push(v);
        }
    }
    pairs
}

// each prime in list concating with b, are prime
fn all_primes(primes_list: &[u64], b: u64) -> bool {
    for &a in primes_list.iter() {
        if !is_concat_primes(a, b) || !is_concat_primes(b, a) {
            return false;
        }
    }
    true
}

// concating prime a and prime b, the result is prime?
// for example, 7, 109 => 7109 is prime, so return true
fn is_concat_primes(a: u64, b: u64) -> bool {
    let c = format!("{}{}", a, b);
    let c = c.parse::<u64>().unwrap();
    primes::is_prime(c)
}
