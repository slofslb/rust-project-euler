use std::time::SystemTime;

const PRIME: u64 = 1_000_000_007_u64;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref ARRAY: Vec<u64> = {
        println!("initializing ARRAY ...");
        let mut arr = vec![1];
        let mut x = 1;
        for _i in 1..PRIME - 1 {
            x = x * 10 % PRIME;
            arr.push(x as u64);
        }
        arr
    };
}

fn main() {
    let start = SystemTime::now();
    let mut result = 0;
    let mut fib = vec![0_u64, 1];
    for i in 2..=90 {
        let n = fib[i - 1] + fib[i - 2];
        fib.push(n);
        let ss = fss(n);
        result = (result + ss) % PRIME;
        println!("n:{} S:{} result: {}", n, ss, result);
    }
    println!("{:?}", start.elapsed());
}
// 922058210

fn ten_power_mod(n: u64) -> u64 {
    let m = n % (PRIME - 1);
    ARRAY[m as usize]
}

fn fs(n: u64) -> u64 {
    let a = n % 9;
    let b = n / 9;
    let s = (a + 1) * ten_power_mod(b) - 1;
    s % PRIME
}

fn sum_group(m: u64) -> u64 {
    let temp = (9 * m) % PRIME;
    let s = 5 * ten_power_mod(m) + PRIME - temp - 5;
    s % PRIME
}

fn fss(n: u64) -> u64 {
    let m = n / 9;
    let mut s = sum_group(m);
    for i in 9 * m..=n {
        s += fs(i);
    }
    s % PRIME
}

/*
fn rr(mod_p: &Vec<u64>, n:u64) -> u64 {
    if n<9 {
        if n == 0 {return 0;}
        if n == 1 {return 1;}
        if n == 2 {return 3;}
        if n == 3 {return 6;}
        if n == 5 {return 15;}
        if n == 8 {return 36;}
    }
    println!("n {}", n);
    let m = (n+1)/9-1;
    let mut r = fss(m);
    println!("m:{}", m);
    for i in (m+1)*9..=n {
        r += fs(mod_p, i);
    }
    r % 1_000_000_007_u64
}

fn fs(mod_p: &Vec<u64>, n:u64) -> u64 {
    let b = n / 9;
    let mut mmm = vec![n%9];
    for i in 1..=b {
        mmm.push(9);
    }

    let v_str = mmm.iter().map(|x| x.to_string()).collect::<String>();
    let bbb = BigUint::from_str(&v_str).unwrap();
    //println!("{:?}", bbb.to_string());
    let ttt = bbb % BigUint::from(1_000_000_007_u64);

    ttt.to_string().parse::<u64>().unwrap()
}

fn fss(m:u64) -> u64 {
    let mut mmm = vec![];
    for i in 0..=m {
        mmm.push(1);
    }

    let v_str = mmm.iter().map(|x| x.to_string()).collect::<String>();
    let bbb = BigUint::from_str(&v_str).unwrap();
    //println!("{:?}", bbb.to_string());
    let t1 = BigUint::from(9_u64) * BigUint::from(m+1);
    let t2 = bbb * BigUint::from(45_u64) - t1;
    let t3 = t2 % BigUint::from(1_000_000_007_u64);

    t3.to_string().parse::<u64>().unwrap()
}
*/
