fn main() {
    let mut sum = 0;
    for a in 1_u32..10000 {
        let b = proper_divisors_sum(a);
        if a != b && proper_divisors_sum(b) == a {
            sum += a;
            println!("{} {}", a, b);
        }
    }
    println!("{}", sum);

    // 用函数式表达，不好理解
    println!(
        "{:?}",
        (1_u32..10000)
            .map(|a| (a, proper_divisors_sum(a)))
            .filter(|&(a, b)| a != b && proper_divisors_sum(b) == a)
            .unzip::<_, _, Vec<_>, Vec<_>>().0
            .iter()
            .sum::<u32>()
    )
}

fn half_factors(num: u32) -> Vec<u32> {
    let s = (num as f32).sqrt() as u32;
    (1..=s).filter(|x| num % x == 0).collect::<Vec<u32>>()
}

// 有点小bug
fn proper_divisors(num: u32) -> Vec<u32> {
    let mut v = half_factors(num);
    for i in (1..v.len()).rev() {
        //不要num自身，所以从1开始
        v.push(num / v[i]);
    }
    v
}

fn proper_divisors_sum(num: u32) -> u32 {
    let divs = proper_divisors(num);
    divs.iter().sum::<u32>()
}
