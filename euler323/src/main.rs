fn main() {
    let mut sum = 0.0;
    for _i in 1..100000 {
        sum += trial() as f64;
    }
    println!("{}", sum / 100000.0);



    let mut e = vec![0.0, 2.0];
    for n in 2..=32 {
        let mut f = 1.0 as f64;
        for r in 1..=n {
            f += comb(n, r) * (1.0 + e[n - r]);
        }
        f = f / (2_f64.powf(n as f64) - 1.0);
        e.push(f);
        println!("E({}) = {:.10}", n, f);
    }

    /*
    for bits in 1..32 {
        let mut sum = 0 as f64;
        for count in 1..100000 {
            sum += trial(bits) as f64;
            //println!("{}", sum / count as f64);
        }
        println!("bits: {}   {}", bits, sum / 100000 as f64);
    }
    */
}

fn trial() -> u32 {
    let mut x = 0_u32;
    for i in 1.. {
        x |= rand::random::<u32>();
        if x == 0xFFFFFFFF {
            return i;
        }
    }
    0
}
// 经过这个简单的测试，大概期望值为6.35左右

/*
fn trial(bits: u32) -> u32 {
    let mut x = 0_u32;
    for i in 1.. {
        x = x | (rand::random::<u32>() % 2_u32.pow(bits));
        //println!("{} {:X}", i, x);
        if x == 2_u32.pow(bits) - 1 {
            // 0xFFFFFFFF {
            //println!("N: {}", i);
            return i;
        }
    }
    0
}
*/

fn comb(n: usize, r: usize) -> f64 {
    let mut c = 1_f64;
    for i in 0..r {
        c *= ((n - i) as f64) / (r - i) as f64;
    }
    c
}
// 6.3551758451
