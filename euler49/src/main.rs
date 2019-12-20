use permutohedron::heap_recursive;

fn main() {
    for x in 1000u64..=9999 {
        if !primes::is_prime(x) {
            continue;
        }
        // 拆成4个数字
        let mut vx: Vec<u64> = x
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut vy = Vec::new();
        heap_recursive(&mut vx, |pt| {
            let y = pt.to_vec().iter().fold(0, |x, a| 10 * x + a);
            if y > x && primes::is_prime(y) && !vy.contains(&y) {
                vy.push(y);
            }
        });
        
        //if vy.len() >= 2 {
        //    println!("{} {:?}", x, vy);
        //}

        for &y in vy.iter() {
            // 按等差关系形成第三个数
            let z = (y - x) + y;
            if vy.contains(&z) && primes::is_prime(z) {
                println!("{}{}{}", x, y, z);
            }
        }
    }
}
