fn main() {
    //println!("2 {}", min_x(2));
    let mut max = 0;
    for d in 2..=1000 {
        let s = (d as f32).sqrt() as usize;
        if s * s == d {
            continue;
        }
        let x = min_x(d);
        if x > max {
            max = x;
            println!("{} {}", d, x);
        }
    }
    println!("{}", max);
}

use num_integer;

fn min_x(d: usize) -> usize {
    for k in 1_usize..=809084421 {
        let t = k * k - 1;
        //if t % d == 0 {
            let t1 = d * t + 1;
            let s = (t1 as f64).sqrt() as usize;
            if s * s == t1 { // d = 61,109,149时会溢出
                let (q, r) = num_integer::div_rem(k * d + s, d - 1);
                if r == 0 {
                    println!("d: {} k: {} x: {}", d, k, q);
                    return q;
                }
            }
        //}
    }
    0
}
