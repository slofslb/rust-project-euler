fn main() {
    //println!("2 {}", min_x(2));
    let mut max = 0;
    for d in (2..=1000).rev() {
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

// https://oeis.org/A002350
// https://en.wikipedia.org/wiki/Pell%27s_equation
// https://en.wikipedia.org/wiki/Chakravala_method
// http://mathshistory.st-andrews.ac.uk/HistTopics/Pell.html
use num_integer;

fn min_x(d: usize) -> u128 {
    for k in 1_u128.. {
        let t1 = (d as u128) * (k * k - 1) + 1;
        let s = (t1 as f64).sqrt() as u128;
        if s * s == t1 {
            // d = 61,109,149时会溢出
            let (q, r) = num_integer::div_rem(k * (d as u128) + s, (d as u128) - 1);
            if r == 0 {
                println!("d: {} k: {} x: {}", d, k, q);
                return q;
            }
        }
    }
    0
}
