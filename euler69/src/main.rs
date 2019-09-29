fn main() {
    let max_check_number = 1_000_000;
    let mut phi = vec![0; max_check_number + 1];

    let mut max_n_phi = 0_f32;
    for n in 2..=10 {
        for i in 1..n {
            if gcd(i, n) == 1 {
                for k in 1..i {
                    phi[k] += 1;
                }
            }
        }
        let n_phi = (n as f32) / (phi as f32);
        if n_phi > max_n_phi {
            println!("n={} n/phi={}", n, n_phi);
            max_n_phi = n_phi;
        }
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
