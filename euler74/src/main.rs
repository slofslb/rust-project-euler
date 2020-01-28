fn main() {
    let mut count = 0;
    for n in 1..1_000_000 {
        let v = non_repeating_chain(n);
        if v.len() == 60 {
            println!("{}", n);
            count += 1;
        }
    }
    println!("count: {}", count);
}
// 402

fn non_repeating_chain(mut n: u64) -> Vec<u64> {
    let mut chain = vec![];
    while !chain.contains(&n) {
        chain.push(n);
        n = sum_fac(n);
    }
    chain
}

// 求各位数字的阶乘之和
fn sum_fac(n: u64) -> u64 {
    // 借鉴第34题，提前计算好0~9的阶乘
    //let fac_0_to_9: Vec<u64> = (0..10).map(|x| factorial(x)).collect();
    let fac_0_to_9 = [1_u64, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
    n.to_string()
        .chars()
        .map(|x| fac_0_to_9[x.to_digit(10).unwrap() as usize])
        .sum()
}
