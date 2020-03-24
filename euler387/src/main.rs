fn main() {
    /* 打印一些数，用于找规律
    for i in 1..10_u64.pow(6) {
        if is_right_truc_harshad(i) {
            println!("{}", i);
        }
    }
    */

    let mut sum = 0;
    for i in 1..=9 {
        sum += harshad_loop(i);
    }
    println!("{}", sum);
}

fn harshad_loop(head: u64) -> u64 {
    if head >= 10_u64.pow(13) {
        return 0;
    }
    let mut sum = 0;
    if is_right_truc_harshad(head) {
        if is_strong_harshad(head) {
            // 素数只能以1，3，5，7，9结尾
            let harshad_primes = (1..=9)
                .step_by(2)
                .map(|i| head * 10 + i)
                .filter(|&x| primes::is_prime(x))
                .collect::<Vec<u64>>();
            if !harshad_primes.is_empty() {
                // println!("{:?}", harshad_primes);
            }
            sum += harshad_primes.iter().sum::<u64>();
        }
        for i in 0..=9 {
            sum += harshad_loop(head * 10 + i);
        }
    }
    sum
}

#[test]
fn test_10000() {
    let h: Vec<u64> = (1..100000000)
        .filter(|x| is_right_truc_strong_harshad_prime(*x))
        .collect();
    println!("{:?}", h);
    assert_eq!(h.iter().sum::<u64>(), 90619);
}

fn sum_of_digits(n: u64) -> u64 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>() as u64
}

// 哈沙德数: 能够被其各位数字和整除
fn is_harshad(n: u64) -> bool {
    n % sum_of_digits(n) == 0
}

// 可右截哈沙德数: 不断截去最后一位数字的结果始终是哈沙德数
fn is_right_truc_harshad(n: u64) -> bool {
    let s = n.to_string();
    let digits = s.chars().map(|c| c.to_digit(10).unwrap() as u64);
    let mut num: u64 = 0;
    let mut sum: u64 = 0;
    for d in digits {
        num = num * 10 + d;
        sum += d;
        if num % sum != 0 {
            return false;
        }
    }
    true
}

use num_integer;
fn is_strong_harshad(n: u64) -> bool {
    let sum_digits = sum_of_digits(n);
    let (q, r) = num_integer::div_rem(n, sum_digits);
    r == 0 && primes::is_prime(q)
}

fn is_right_truc_strong_harshad_prime(n: u64) -> bool {
    let m = n / 10;
    m != 0 && primes::is_prime(n) && is_strong_harshad(m) && is_right_truc_harshad(m)
}
