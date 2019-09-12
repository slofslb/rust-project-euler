fn main() {
    let mut s = 0;
    for n in 2..999999 {
        let sum_pow = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap().pow(5))
            .sum::<u32>();
        if sum_pow == n {
            println!("{}", n);
            s += n;
        }
    }
    println!("sum: {}", s);

    // 第二种写法
    let sum_pow = (2..999999).filter(|&x| is_power_number(x)).sum::<u32>(); //.collect::<Vec<u32>>();
    println!("sum: {:?}", sum_pow);

    // 第三种：写成变态的一行
    println!(
        "sum: {}",
        (2..999999)
            .filter(|&n| n
                == n.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap().pow(5))
                    .sum::<u32>())
            .sum::<u32>()
    );
}
// 443839

fn is_power_number(n: u32) -> bool {
    n == n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(5))
        .sum::<u32>()
}
