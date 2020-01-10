extern crate num_bigint;
use num_bigint::BigUint;

fn main() {
    // 先把一些阶乘计算好，保存起来
    let mut fact = vec![BigUint::from(1 as u64); 101];
    let mut a = BigUint::from(1 as u64);
    for n in 2..=100 {
        a *= BigUint::from(n as u64);
        fact[n] = a.clone();
    }
    //println!("{:?}", fact);

    let mut s = 0;
    for n in 1..=10 {
        let mut b = BigUint::from(1_u64);
        for r in 1..=n {
            let comb = &fact[n] / &fact[r] / &fact[n - r];
            b *= &comb;
            //println!("{} {} {}", n, r, comb.to_string());
        }
        let bb = b.to_string().parse::<u64>().unwrap();
        //println!("B({}) = {}", n, bb);

        let f_all = primes::factors(bb);
        let f_uniq = primes::factors_uniq(bb);
        //println!("{:?}", f_all);
        //println!("{:?}", f_uniq);

        let mut d = 1;
        for f in f_uniq {
            let c = f_all.iter().filter(|&n| *n == f).count();
            d *= (f.pow(c as u32 + 1) - 1) / (f-1);
        }
        //println!("D({}) = {}", n, d);

        s += d;
        println!("S({}) = {}", n, s);
    }
}
