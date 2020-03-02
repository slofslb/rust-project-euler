fn main() {
    println!("{}",336108797689267467_u128-8191 );
    let mut cache = vec![0_u8;100000000];
    for b in 2_u128.. {
        let n = ((cache.len() as f64) * (b as f64 - 1.0) + 1.0).log10() / (b as f64).log10();
        let n = n as u32;
        if n == 2 {
            break;
        }
        for i in 3..=n {
            let x = (b.pow(i)-1)/(b-1);
            if x == 8191 {
                println!("{} {} {}", b, i, x);
            }
            if cache[x as usize] >= 1 {
                println!("{} {} {}", b, i, x);
                //break;
            }
            cache[x as usize] += 1;

        }
    }


    let limit = 10_u128.pow(12);
    let mut total :u128= 0;
    for b in 2_u128.. {
        let n = ((limit as f64) * (b as f64 - 1.0) + 1.0).log10() / (b as f64).log10();
        let n = n as u32;
        if n == 2 {
            break;
        }
        let sum = (b*(b.pow(n) -1)/(b-1)-(n as u128))/(b-1);
        let sum = sum - (b+1) - 1;
        total += sum as u128;
        println!("b:{} n:{} {} {}", b, n, sum, total);
    }
    total -= 30;
    println!("{}", total);
    /*
    for n in 1..12 {
        print!("{}", n);
        for b in 2_u64..16 {
            let t = (b.pow(n) - 1) / (b - 1);
            print!("{:12}", t);
        }
        println!("");
    }
    */
}
