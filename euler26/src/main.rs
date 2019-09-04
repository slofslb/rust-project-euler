fn main() {
    let mut max_cycle: u32 = 0;
    for n in 2..1000 {
        let rc = reciprocal_cycle(n);
        if rc > max_cycle {
            println!("n={} cycle={}", n, rc);
            max_cycle = rc;
        }
    }
    println!("max reciprocal cycle: {}", max_cycle);
}

fn reciprocal_cycle(d: u32) -> u32 {
    let mut remainders: Vec<u32> = vec![1]; //余数
    let mut numerator = 1; //分子
    while {
        numerator = numerator * 10 % d;
        numerator != 0
    } {
        let pos = remainders.iter().position(|&x| x == numerator);
        match pos {
            Some(x) => {
                //余数重复出现时
                return (remainders.len() - x) as u32;
            }
            None => {
                remainders.push(numerator);
            }
        }
    }
    0 //除尽的时候，表示循环节为0
}


fn reciprocal_cycle_v1(d: u32) -> u32 {
    let mut remainders: Vec<u32> = vec![1]; //余数
    let mut digits: Vec<u32> = vec![]; //商

    let mut numerator = 1; //分子
    while numerator != 0 {
        digits.push(numerator * 10 / d);
        numerator = numerator * 10 % d; //余数
        let pos = remainders.iter().position(|&x| x == numerator);
        match pos {
            Some(x) => {
                //余数重复出现时
                return (digits.len() - x) as u32;
            }
            None => {
                remainders.push(numerator);
            }
        }
    }
    0 //除尽的时候，表示循环节为0
}