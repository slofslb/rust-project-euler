use primes::PrimeSet;

fn main() {
    let mut max_ratio_n_phi = 0_f32;
    for n in 2..=3000 {
        let mut phi = 0;
        for i in 1..n {
            // 最大公约数为1，表示互质
            if gcd(i, n) == 1 {
                phi += 1;
            }
        }
        let ratio_n_phi = (n as f32) / (phi as f32);
        if ratio_n_phi > max_ratio_n_phi {
            println!("n= {}  n/phi= {}", n, ratio_n_phi);
            max_ratio_n_phi = ratio_n_phi;
        }
    }
    let mut pset = PrimeSet::new();
    let mut prod = 1;
    for p in pset.iter() {
        prod *= p;
        if prod > 1_000_000 {
            break;
        }
        println!("{}", prod);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}