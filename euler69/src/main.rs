use hashbrown::HashMap;

fn main() {
    let mut cache :HashMap<(u64,u64), u64> = HashMap::new();

    let mut max_n_phi = 0_f32;
    for n in 2..=10 {//1_000_000 {
        if n % 10000 == 0{
            println!("{}", n);
        }
        let mut phi = 0;
        for i in 1..n {
            if primes::is_prime(n) || primes::is_prime(i)
             || gcd(&mut cache, i, n) == 1 {
                phi += 1;
            }
        }
        let n_phi = (n as f32) / (phi as f32);
        if n_phi > max_n_phi {
            println!("n={} n/phi={}", n, n_phi);
            max_n_phi = n_phi;
        }
    }
}

fn gcd(cache: &mut HashMap<(u64, u64), u64>, a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        if cache.contains_key(&(a, b)) {
            //println!("cache! {:?}", (a,b) );
            return *cache.get(&(a,b)).unwrap();
        }
        let temp = gcd(cache, b, a % b);
        if a < b && b < 100000 {
            cache.insert((a,b), temp);
        }
        temp
    }
}
