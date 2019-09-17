/*
折腾的过程：
1）直接按照题意，可以非常容易写出来，10000以内很快
但数值大了之后，就会溢出
2）改算法，提前生成素数表，但当数字大的时候，生成素数都非常花时间，还是算不出来，重复计算的地方太多
3）梳理思路，把前面的一些数打印出来，分析规律，发现满足要求的数并不多
前面的字头确定了之后，后面不会跑出那些范围
4）可以用递归的方法，也不用提前生成素数，临时判断就行
release模式，几秒算完，算法的力量
*/
fn main() {
    for i in 1..=9 {
        right_har(i);
    }
}

fn right_har(start: u64) {
    if start >= 9_999_999_999_999 {
        return;
    }
    if right_harshad(start) {
        if strong(start as usize) {
            //println!("{}", start);
            for j in (1..=9).step_by(2) {
                let temp = start * 10 + j;
                if primes::is_prime(temp) {
                    println!("{}", temp);
                    //sum += temp;
                }
            }
        }
        for i in 0..=9 {
            right_har(start * 10 + i);
        }
    }
}

fn right_harshad(n: u64) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num: u64 = 0;
    let mut sum: u64 = 0;
    for d in digits {
        num = num * 10 + d as u64;
        sum += d as u64;
        if num % sum != 0 {
            return false;
        }
    }
    true
}

fn strong(n: usize) -> bool {
    let digits: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut num: u64 = 0;
    let mut sum: u64 = 0;
    for d in digits {
        num = num * 10 + d as u64;
        sum += d as u64;
        if num % sum != 0 {
            return false;
        }
    }
    primes::is_prime((n as u64) / (sum as u64))
}
