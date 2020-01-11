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

        let mut f_all = primes::factors(bb);
        let f_uniq = primes::factors_uniq(bb);
        println!("{:?}", f_all);
        println!("{:?}", f_uniq);
        vec_remove(&mut f_all, &f_uniq);
        println!("{:?}", f_all);

        let mut d = 1;
        for f in f_uniq {
            let c = f_all.iter().filter(|&n| *n == f).count();
            d *= (f.pow(c as u32 + 1) - 1) / (f-1);
        }
        //println!("D({}) = {}", n, d);

        s += d;
        println!("S({}) = {}", n, s);
    }

    let rrr = factors_b(5);
    println!("{:?}", rrr);

    let temp = factors_sum(&rrr);
    println!("{:?}", temp);

    let mut s = 0;
    for n in 1..=10 {
        let rrr = factors_b(n);

        let map = factors_to_hash_map(&rrr);
        println!("{:?}", map);
    
        let temp = factors_sum(&rrr);
        //println!("D({}) = {:?}", n, temp);

        s = (s + temp) % 1_000_000_007_u64;
        println!("S({}) = {}", n, s);
    }    
}

fn factors_sum(v: &Vec<u64>) -> u64 {
    let mut uniq = v.clone();
    uniq.dedup();

    let mut prod = BigUint::from(1_u64);
    for p in uniq {
        let c = v.iter().filter(|&x| *x == p).count() as u64;
        let t = (big_pow(p, c+1) - BigUint::from(1_u64)) / (BigUint::from(p-1));
        //print!("({} ^ {}) ", p, c);
        prod = (prod * t);// % 1_000_000_007_u64;
    }
    //println!("");
    let prod = prod % 1_000_000_007_u64;
    prod.to_string().parse::<u64>().unwrap()
}

fn big_pow(a:u64, b:u64) -> BigUint {
    let mut prod = BigUint::from(1 as u64);
    for _i in 0..b {
        prod *= BigUint::from(a as u64);
    }
    prod
}



fn factors_b(n:u64) -> Vec<u64>{
    let mut factors = vec![];
    for i in 1..=n {
        let mut r = i;
        if n-r < i {
            r = n-r;
        }
        let mut f = comb_factors(n, r);
        factors.append(&mut f);
    }
    factors.sort();
    factors.to_vec()
}


// va中元素已经排好序
fn vec_remove(va: &mut Vec<u64>, vb:&Vec<u64>){
    for item in vb {
        let index = va.binary_search(&item).unwrap();
        //println!("{:?} {:?} {}", va, vb, index);
        va.remove(index);
    }
}

fn comb_factors(m:u64, n:u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut x = m;
    for i in 0..n {
        let mut f = primes::factors(x);
        factors.append(&mut f);
        x -= 1;
    }
    factors.sort();
    //println!("{:?}", factors);
    for i in 2..=n {
        let f = primes::factors(i);
        //println!("{} {:?}", n, f);
        vec_remove(&mut factors, &f);
    }
    factors.to_vec()
}

use std::collections::HashMap;

/* error
fn comb_factors_hash_map(m:u64, n:u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    let mut lastmap = HashMap::new();
    for x in 1..n {
        let mut f = primes::factors(x);
        lastmap = factors_to_hash_map(&f);
        factors.append(&mut f);
        x -= 1;
    }
    factors.sort();
    //println!("{:?}", factors);
    for i in 2..=n {
        let f = primes::factors(i);
        //println!("{} {:?}", n, f);
        vec_remove(&mut factors, &f);
    }
    factors.to_vec()
}
*/

fn factors_to_hash_map(factors:&Vec<u64>) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    for f in factors {
        let v = map.get(f).cloned(); // 如果不写cloned()，有警告，不理解原因
        match v {
            Some(x) => {
                map.insert(*f, x+1);
            }
            None => {
                map.insert(*f, 1);
            }
        }
    }
    map
}