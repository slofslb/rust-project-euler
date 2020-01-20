extern crate num_bigint;
use num_bigint::BigUint;

// use num_traits::pow;
use num_traits::pow;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref FACTORS: Vec<HashMap<u64,u64>> = {
        let mut factors_all = vec![HashMap::new()];
        for i in 1..=20000 {
            let f = primes::factors(i);
            let a = factors_to_hash_map(&f);
            factors_all.push(a);
        }
        factors_all
    };

    static ref FF: Vec<HashMap<u64,u64>> = {
        let mut v = vec![HashMap::new()];
        for i in 1..=20000 {
            let mut map = v[i-1].clone();
            let f = primes::factors(i as u64);
            let a = factors_to_hash_map(&f);
            hash_map_add(&mut map, &a);
            v.push(map);
        }
        //println!("{:?}", v);
        v
    };

    static ref POWS: HashMap<u64, Vec<BigUint>> = {
        let mut map = HashMap::new();
        for i in 2..20000 {
            if primes::is_prime(i) {
                let mut v = vec![];
                v.push(BigUint::from(i as u64));
                let mut prod = BigUint::from(i as u64);
                for j in 0..15 {
                    prod = &prod * &prod;
                    v.push(prod.clone());
                }
                map.insert(i, v);
            }
        }
        map
    };
}

fn main() {
    println!("{:?}", POWS.get(&2));


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

    let mut cache_pow = vec![0_u64; 20000];
    let mut cache_value = vec![BigUint::from(1_u64); 20000];

    let mut map = HashMap::new();
    let mut s = 1;
    for n in 2..=20000 {
        /*
        let f = primes::factors(n);
        let factor_n = factors_to_hash_map(&f);
        */
        let factor_n = &FACTORS[n as usize];
        hash_map_add_count(&mut map, &factor_n, n-1);
        /*
        for i in 0..n-1 {
            hash_map_add(&mut map, &factor_n);
        }
        */
        let m = &FF[n as usize - 1];
        hash_map_substract(&mut map, &m);
        /*
        for i in 2..=n-1 {
            let f = primes::factors(i);
            let m = factors_to_hash_map(&f);
            hash_map_substract(&mut map, &m);
        }
        */
    
        //let d = factors_hash_map_sum(&map);
        let d = factors_hash_map_sum(&map, &mut cache_pow, &mut cache_value);
        //println!("D({}) = {:?}", n, d);
        
        s = (s + d) % 1_000_000_007_u64;
        if n == 10 {
            assert_eq!(s, 141740594713218418 % 1000000007_u64);
        }
        if n == 100 {
            assert_eq!(s, 332792866_u64);
        }
        if n % 100 == 0 {
            println!("{:?}", &map);
            println!("S({}) = {}", n, s);
        }
    }    

    //let map = comb_factors_hash_map(5);
    //println!("{:?}", map);

}

fn factors_sum(v: &Vec<u64>) -> u64 {
    let mut uniq = v.clone();
    uniq.dedup();

    let mut prod = BigUint::from(1_u64);
    for p in uniq {
        let c = v.iter().filter(|&x| *x == p).count() as usize;
        let t1 = pow(BigUint::from(p), c+1);
        let t = (t1 - BigUint::from(1_u64)) / (BigUint::from(p-1));
        //let t = (big_pow(p, c+1) - BigUint::from(1_u64)) / (BigUint::from(p-1));
        //print!("({} ^ {}) ", p, c);
        prod = (prod * t);// % 1_000_000_007_u64;
    }
    //println!("");
    let prod = prod % 1_000_000_007_u64;
    prod.to_string().parse::<u64>().unwrap()
}

fn big_pow(a:u64, b:u64) -> BigUint {
    let bin = format!("{:b}", b);
    let mut prod = BigUint::from(1 as u64);
    //let mut p = BigUint::from(a as u64);
    let pp = POWS.get(&a).unwrap();
    let mut j = 0;
    for i in bin.chars().rev() {
        if i == '1' {
            prod *= &pp[j];
        }
        j += 1;
        //p = &p * &p;
    }
    prod
    /*
    let mut prod = BigUint::from(1 as u64);
    for _i in 0..b { //slowly
        prod *= BigUint::from(a as u64);
    }
    return prod;
    */
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


fn factors_hash_map_sum2(map: &HashMap<u64, u64>) -> u64 {
    let mut prod = BigUint::from(1_u64);
    for (&f, count) in map {
        let t = (big_pow(f, count+1) - BigUint::from(1_u64)) / (BigUint::from(f-1));
        //print!("({} ^ {}) ", p, c);
        prod = prod * t;// % 1_000_000_007_u64;
    }
    //println!("");
    let prod = prod % 1_000_000_007_u64;
    prod.to_string().parse::<u64>().unwrap()
}

fn factors_hash_map_sum(map: &HashMap<u64, u64>, cache_pow:&mut Vec<u64>, cache_value:&mut Vec<BigUint>) -> u64 {
    let mut prod = BigUint::from(1_u64);
    for (&f, count) in map {
        let p = cache_pow[f as usize];
        let mut new_v = cache_value[f as usize].clone();
        //println!("{} {} {:?}", f, p, new_v.to_string() );
        if count > &p {
            /*
            for i in 0..count - p {
                new_v = new_v * BigUint::from(f);
            }
            */
            new_v = new_v * big_pow(f, count-p);
        }
        else if count < &p {
            /*
            for i in 0..p - count {
                new_v = new_v / BigUint::from(f);
            } 
            */  
            new_v = new_v / big_pow(f, p-count);
        }
        else {

        }
        //println!("f:{} count:{} new_v:{:?}", f, count, new_v.to_string() );
        cache_pow[f as usize] = *count;
        cache_value[f as usize] = new_v.clone();

        let t = (new_v * BigUint::from(f) - BigUint::from(1_u64)) / (BigUint::from(f-1));

//        let t = (big_pow(f, count+1) - BigUint::from(1_u64)) / (BigUint::from(f-1));
        //print!("({} ^ {}) ", p, c);
        prod = prod * t;// % 1_000_000_007_u64;
    }
    //println!("");
    let prod = prod % 1_000_000_007_u64;
    prod.to_string().parse::<u64>().unwrap()

}

fn comb_factors_hash_map(x:u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();

    let mut count = x as i64 - 1;
    for n in (2..=x).rev() {
        
        //let f = primes::factors(n);
        //let a = factors_to_hash_map(&f);
        let a = &FACTORS[n as usize];
        if count >= 0 {
            hash_map_add_count(&mut map, &a, count as u64);
            /*
            for i in 0..count {
                hash_map_add(&mut map, &a);
            }
            */
            //println!("n={} add {} {:?}",n, count, map);
        }
        else {
            hash_map_substract_count(&mut map, &a, count.abs() as u64);
            /*
            for i in 0..count.abs() {
                hash_map_substract(&mut map, &a);
            }
            */
            //println!("n={} substract {} {:?}", n, count, map);
        }
        count -= 2;
    } 
    map
}

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



fn hash_map_add(map:&mut HashMap<u64, u64>, a:&HashMap<u64, u64>) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x+count);
            }
            None => {
                map.insert(*f, *count);
            }
        }
    }
} 

fn hash_map_add_count(map:&mut HashMap<u64, u64>, a:&HashMap<u64, u64>, times:u64) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x+count*times);
            }
            None => {
                map.insert(*f, *count * times);
            }
        }
    }
} 

fn hash_map_substract(map:&mut HashMap<u64, u64>, a:&HashMap<u64, u64>) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x-count);
            }
            None => {
            }
        }
    }
} 
fn hash_map_substract_count(map:&mut HashMap<u64, u64>, a:&HashMap<u64, u64>, times:u64) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x - count * times);
            }
            None => {
            }
        }
    }
} 


