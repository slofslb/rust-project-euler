fn main() {
    let mut count = 0;
    let mut v:Vec<(u64, u64)> = vec![];
    for d in 2..=1000 {
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
    let g = num::integer::gcd(origin_a, origin_b);
    (origin_a / g, origin_b / g)
} 
