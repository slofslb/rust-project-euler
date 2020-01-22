use std::collections::HashMap;

const MODULUS: u64 = 1_000_000_007_u64;

fn main() {
    let mut factorial_factors = HashMap::new();
    let mut map = HashMap::new();
    let mut s = 1;
    for n in 2..=20000 {
        let factor_n = factors_map(n);
        map_add_count(&mut map, &factor_n, n);

        // 缓存 n! 阶乘的因子
        map_add(&mut factorial_factors, &factor_n);

        map_substract(&mut map, &factorial_factors);

        //println!("{:?}", &map);
        let d = map_sum(&map);
        
        s = (s + d) % MODULUS;
        if n == 10 {
            assert_eq!(s, 141740594713218418 % MODULUS);
        }
        if n == 100 {
            assert_eq!(s, 332792866_u64);
        }
        if n % 100 == 0 {
            println!("D({}) = {:?} \t S({}) = {}", n, d, n, s);
        }
    }
    println!("{}", s);

}
// 538319652

// http://rosettacode.org/wiki/Modular_inverse#Rust
fn mod_inv(a: isize, module: isize) -> isize {
    let mut mn = (module, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += module;
    }
    xy.0
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}


fn map_sum(map: &HashMap<u64, u64>) -> u64 {
    let mut prod = 1;
    for (&f, count) in map {
        if *count > 0 {
            let temp = mod_pow(f, count + 1, MODULUS) - 1;
            let inv = mod_inv(f as isize - 1, MODULUS as isize) as u64;
            //println!("inv:{}", inv);
            let temp = temp * inv % MODULUS;
            prod = prod * temp % MODULUS;
            //println!("f:{} count+1:{} prod: {}", f, count+1, prod);
        }
    }
    prod
}

fn factors_map(n:u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    let all_factors = primes::factors(n);
    for f in &all_factors {
        let v = map.get(f).cloned(); // 如果不写cloned()，有警告，不理解原因
        match v {
            Some(x) => {
                map.insert(*f, x + 1);
            }
            None => {
                map.insert(*f, 1);
            }
        }
    }
    map
}

fn map_add(map: &mut HashMap<u64, u64>, a: &HashMap<u64, u64>) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x + count);
            }
            None => {
                map.insert(*f, *count);
            }
        }
    }
}

fn map_add_count(map: &mut HashMap<u64, u64>, a: &HashMap<u64, u64>, times: u64) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x + count * times);
            }
            None => {
                map.insert(*f, *count * times);
            }
        }
    }
}

fn map_substract(map: &mut HashMap<u64, u64>, a: &HashMap<u64, u64>) {
    for (f, count) in a {
        let v = map.get(f).cloned();
        match v {
            Some(x) => {
                map.insert(*f, x - count);
            }
            None => {}
        }
    }
}

