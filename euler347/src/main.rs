use std::collections::HashMap;

fn main() {
    let mut sieve = vec![false; 10_000_001_usize];
    let mut sum = 0;
    for n in (2..=10_000_000_u64).rev() {
        if sieve[n as usize] {
            continue;
        }
        let factors = primes::factors_uniq(n);
        if factors.len() == 2 {
            sum += n;
            let inc = factors[0]*factors[1]; 
            let mut i = inc;
            while i < n {
                 sieve[i as usize] = true;
                 i += inc;
            }
        }
    }
    println!("{:?}", sum);
}
// 11109800204052


fn main_v0() {
    let mut hashmap = HashMap::new();
    for n in (2..=10_000_000_u64).rev() {
        let factors = primes::factors_uniq(n);
        if factors.len() == 2 {
            let key = (factors[0], factors[1]);
            hashmap.entry(key).or_insert(n);
            /*
            if !hashmap.contains_key(&key) {
                hashmap.insert(key, n);
            }
            */
        }
    }
    //println!("{:?}", hashmap);
    let s: u64 = hashmap.iter().map(|(_, &v)| v).sum();
    println!("{:?}", s);
}



