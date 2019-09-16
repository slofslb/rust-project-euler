use primes::PrimeSet;

fn main() {
    let mut pset = PrimeSet::new();
    let pp: Vec<u64> = pset.iter().take(1000).collect();
    //println!("{:?}", pp);

    let mut max_n_phi = 0_f32;
    for n in (2..=100000).step_by(2) {
        //1_000_000 {
        let mut v_phi = vec![1];
        for p in &pp {
            if *p >= n {break;}
            if gcd(*p, n) == 1 {// 互质
                for i in (*p..n).step_by(*p as usize) {
                    if !v_phi.contains(&i) {
                        v_phi.push(i);
                    } 
                }
                let n_phi = (n as f32) / (v_phi.len() as f32);
                if n_phi < max_n_phi {
                    break;
                }
            }
        }
        let n_phi = (n as f32) / (v_phi.len() as f32);
        if n_phi > max_n_phi {
            println!("n={} n/phi={}", n, n_phi);
            max_n_phi = n_phi;
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        let temp = gcd(b, a % b);
        temp
    }
}
