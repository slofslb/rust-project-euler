use primes::PrimeSet;
use std::collections::HashMap;

fn main() {
    // 最早的试验
    println!("16 {}", min_num_has_factors(16));
    println!("32 {}", min_num_has_factors(32));
    println!("64 {}", min_num_has_factors(64));
    println!("128 {}", min_num_has_factors(128));

    println!("{}", p500(500500));

    /*
    for m in 1..15 {
        let divisors = 2_u64.pow(m);
        let mm = min_n(divisors);
        println!("divisors: {} n:{}", divisors, mm);
    }
    */
}
// 35407281

#[test]
fn p_16_factors() {
    // 16 divisors = 2^4, so call p500(4)
    assert_eq!(120, p500(4));


}

fn p500(n: usize) -> u64 {
    let mut pset = PrimeSet::new();
    let primes: Vec<_> = pset.iter().take(n).collect();
    //println!("{:?}", primes);

    let mut b = vec![];
    for _i in 1..=n {
        let mut min = (primes[b.len()] as f64).log10();
        let mut min_j = b.len();
        for j in 0..b.len() {
            let temp = 2_f64.powf(b[j] as f64) * (primes[j] as f64).log10();
            if temp < min {
                min_j = j;
                min = temp;
            }
            if b[j] == 1 {
                break;
            }
        }
        if min_j == b.len() {
            b.push(1);
        } else {
            b[min_j] += 1;
        }
    }

    //println!("{} {:?} ...", b.len(), &b[..10]);

    let mut result = 1_u64;
    for i in 0..b.len() {
        let exp = 2_u32.pow(b[i]) - 1;
        for _j in 0..exp {
            result = result * primes[i] % 500500507;
        }
    }
    result
}

use itertools::Itertools;
fn min_num_has_factors(x: u64) -> u64 {
    for n in 2.. {
        let factors = primes::factors(n);
        let map = factors_to_hash_map(&factors);

        let factors_num: u64 = map.values().map(|x| x + 1).product();
        if factors_num == x {
            print!("{} = ", n);
            println!("{}", &map.iter().map(|(k,v)| k.to_string() + &"^" + &v.to_string()).join(" * "));
            println!("{} = {}", factors_num, &map.values().map(|x| "(".to_string() + &x.to_string() + &"+1)").join(" * "));
            return n;
        }
    }
    0
}

fn factors_to_hash_map(factors: &[u64]) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    for f in factors {
        let v = map.get(f).cloned(); // 如果不写cloned()，有警告，不理解原因
        match v {
            Some(x) => {
                map.insert(*f, x + 1);
            }
            None => {
                map.insert(*f, 1);
            }
        }
    }
    map
}
