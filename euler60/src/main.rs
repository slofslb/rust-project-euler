use primes::PrimeSet;
fn main() {
    let max = 10000;
    let primes = vec![];
    let p1 = prime_pairs(max, &primes);
    for p in p1 {
        let p2 = prime_pairs(max, &p);
        for q in p2 {
            let p3 = prime_pairs(max, &q);
            for r in p3 {
                let p4 = prime_pairs(max, &r);
                for s in p4 {
                    let p5 = prime_pairs(max, &s);
                    if !p5.is_empty() {
                        println!("{:?}", p5);
                        println!("{:?}", p5[0].iter().sum::<u64>());
                    }
                }
            }
        }
    }
}
// error: 129976621
// right: 26033 = [13, 5197, 5701, 6733, 8389]

fn prime_pairs(max: u64, primes_list: &[u64]) -> Vec<Vec<u64>> {
    let mut pairs = vec![];

    let mut pset1 = PrimeSet::new();
    for j in pset1.iter() {
        if j == 2 || j == 5 {
            continue;
        }
        if j > max {
            break;
        }
        if primes_list.is_empty() {
            pairs.push(vec![j]);
            continue;
        }
        if j <= *primes_list.last().unwrap() {
            continue;
        }
        if all_primes(primes_list, j) {
            let mut v = primes_list.to_vec();
            v.push(j);
            pairs.push(v);
        }
    }
    pairs
}

fn all_primes(primes_list: &[u64], b: u64) -> bool {
    for a in primes_list.iter() {
        let c = format!("{}{}", a, b);
        let c = c.parse::<u64>().unwrap();
        if !primes::is_prime(c) {
            return false;
        }
        let d = format!("{}{}", b, a);
        let d = d.parse::<u64>().unwrap();
        if !primes::is_prime(d) {
            return false;
        }
    }
    true
}

fn is_concat_primes(a: u64, b: u64) -> bool {
    let c = format!("{}{}", a, b);
    let c = c.parse::<u64>().unwrap();
    if !primes::is_prime(c) {
        return false;
    }
    let d = format!("{}{}", b, a);
    let d = d.parse::<u64>().unwrap();
    primes::is_prime(d)
}
