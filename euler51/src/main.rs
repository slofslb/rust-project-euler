//use primes::PrimeSet;

fn main() {
    
    let mut vp = vec![];
    //let mut pset = PrimeSet::new();
    for p in 10000_u32..99999 {
        if primes::is_prime(p as u64) {
            vp.push(p);
        }
    }
    //println!("{:?}", vp );

    let i = 1;
    let j = 2;
    let mut vp1 = vec![];
    for p in vp {
       let a = remove(p, i);
       vp1.push( remove(a, j));
    }
    //println!("{:?}", vp1 );

    let mut count :Vec<u32>= vec![0; 99999];
    for p in &vp1 {
        count[*p as usize] += 1;
    }
    println!("{:?}", count );

    vp1.clear();
    for p in 10000_u32..99999 {
        if count[p as usize] >= 7 {
            vp1.push(p);
        }
    }
    println!("{:?}", vp1 );
/*
    let mut vp2 = vec![];
    for p in vp1 {
        vp2.push(remove(p, j));
    }
*/
}

fn remove(n:u32, i:u32) -> u32 {
    let d = n / 10_u32.pow(i) % 10; 
    n - d * 10_u32.pow(i)
}
