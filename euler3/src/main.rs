fn main() {
    let big_num = 600_851_475_143;
    let mut i = (big_num as f64).sqrt() as u64;
    while i > 0 {
        if big_num % i == 0 && is_prime(i) {
            println!("{}", i);
            break;
        }
        i -= 1;
    }
    if i == 1 {
        println!("not found");
    }
    // 另一种方法，用primes函数库
    println!("{:?}", primes::factors_uniq(600_851_475_143).last().unwrap());

    //main_slow();
}
// 6857

fn is_prime(num: u64) -> bool {
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main_slow() {
    let big_num = 600_851_475_143;
    for i in (1..=big_num / 2).rev() {
        if big_num % i == 0 && is_prime(i) {
            println!("{}", i);
            break;
        }
    }
}
