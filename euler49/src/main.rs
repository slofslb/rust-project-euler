use permutohedron::heap_recursive;

fn main() {
    for x in 1000u64..9999 {
        if !primes::is_prime(x) {
            continue;
        }
        let mut vx: Vec<u64> = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();
        //println!("{:?}", vx);

        let mut ys = Vec::new();
        heap_recursive(&mut vx, |pt| {
            let perm_x = pt
                .to_vec()
                .iter()
                .map(|p| p.to_string())
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            ys.push(perm_x)
        });
        //println!("{:?}", ys);

        for &y in ys.iter() {
            if y > x && primes::is_prime(y) {
                let z = (y - x) + y;
                if ys.contains(&z) && primes::is_prime(z) {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
}
