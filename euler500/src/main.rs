use itertools::Itertools;
use primes::PrimeSet;
fn main() {
    // 最早的试验
    min_number_has_factors(4); // 2^2
    min_number_has_factors(8); // 2^3
    min_number_has_factors(16); // 2^4
    min_number_has_factors(32); // 2^5
    min_number_has_factors(64); // 2^6
    min_number_has_factors(128); // 2^7
    min_number_has_factors(256); // 2^8

    println!("{}", p500(500500));
}
// 35407281

#[test]
fn p_16_factors() {
    // 16 divisors = 2^4, so call p500(4)
    assert_eq!(120, p500(4));
}

fn p500(n: usize) -> u64 {
    let mut pset = PrimeSet::new();
    let primes: Vec<_> = pset.iter().take(n).collect();
    let primes_log: Vec<_> = primes.iter().map(|x| (*x as f64).log10()).collect();
    //println!("{:?}", primes);

    let mut b = vec![1];
    for _i in 2..=n {
        let mut min = primes_log[b.len()];
        let mut pos = b.len(); // 默认尾部增加一个
        for j in 0..b.len() {
            let temp = 2_f64.powf(b[j] as f64) * primes_log[j];
            if temp < min {
                pos = j;
                min = temp;
            }
            if b[j] == 1 {
                break;
            }
        }
        if pos == b.len() {
            b.push(1);
        } else {
            b[pos] += 1;
        }
    }

    //println!("{} {:?} ...", b.len(), &b[..10]);

    let mut result = 1_u64;
    for i in 0..b.len() {
        let exp = 2_u32.pow(b[i]) - 1;
        for _j in 0..exp {
            result = result * primes[i] % 500500507;
        }
    }
    result
}

fn min_number_has_factors(x: u64) -> u64 {
    for n in 2.. {
        let groups = factors_group(n);
        let factors_num = groups.iter().map(|(_, x)| x + 1).product::<u64>();
        if factors_num == x {
            println!("{}, divisors num: {}", n, factors_num);
            print_factors_group(groups);
            return n;
        }
    }
    0
}

// 如果一个数有这些因子：[2, 2, 3, 3, 3, 3, 5, 7]
// 则得到：[(2,2), (3,4), (5,1), (7,1)]
fn factors_group(n: u64) -> Vec<(u64, u64)> {
    let factors = primes::factors(n);
    let groups = factors
        .iter()
        .group_by(|e| **e)
        .into_iter()
        .map(|(k, group)| (k, group.count() as u64))
        .collect::<Vec<(u64, u64)>>();
    groups
}

fn print_factors_group(groups: Vec<(u64, u64)>) {
    println!(
        "{}",
        &groups
            .iter()
            .map(|(k, v)| k.to_string() + &"^" + &v.to_string())
            .join(" * ")
    );
    println!(
        "因子个数:  {}",
        &groups
            .iter()
            .map(|(_, v)| "(".to_string() + &v.to_string() + &"+1)")
            .join(" * ")
    );
}
