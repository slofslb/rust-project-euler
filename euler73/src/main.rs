fn main() {
    let mut count = 0;
    let mut v:Vec<(u64, u64)> = vec![];
    for d in 2..=100 {
        for n in 1..d {
            if 2 * n < d && d < 3 * n {
                let r = gcd(n, d);
                if !v.contains(&r) {
                    v.push(r);
                    //println!("{:?} ", r);
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}

fn gcd(origin_a:u64, origin_b:u64) -> (u64, u64) {
    let mut a = origin_a;
    let mut b = origin_b;
    while b != 0 {
		let t = a % b;
		a = b;
		b = t;
    }
    (origin_a / a, origin_b / a)
} 
