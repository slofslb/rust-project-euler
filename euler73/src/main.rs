fn main() {
    // version 0.1
    /*
    let mut count = 0;
    let mut v: Vec<(usize, usize)> = vec![];
    for d in 2..=1200 {
        for n in 1..d {
            if 2 * n < d && d < 3 * n {
                let g = num::integer::gcd(n, d);
                let r = (n / g, d / g);
                if !v.contains(&r) {
                    v.push(r);
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
    */

    let mut exclude_list = vec![vec![]; 12001];

    let mut count = 0;
    for d in 2..=12000 {
        for n in 1..d {
            if 2 * n < d && d < 3 * n && !exclude_list[d].contains(&n) {
                count += 1;
                for i in 2..=12000 / d {
                    exclude_list[d * i].push(n * i);
                }
            }
        }
    }
    println!("{}", count);
}
// 7295372


/*
use std::cmp::{min, max};
// https://rosettacode.org/wiki/Greatest_common_divisor#Stein.27s_Algorithm
fn gcd2(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y               => y,
        ((0, x), _) | ((x, 0), _)           => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd2(x >> 1, y),
        ((x, y), (0, 0))                    => gcd2(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1))                    => { let (x, y) = (min(x, y), max(x, y));
                                                 gcd2((y - x) >> 1, x)
                                               }
        _                                   => unreachable!(),
    }
}

*/
