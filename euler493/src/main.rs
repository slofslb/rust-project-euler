use rand::{seq, thread_rng, Rng};
extern crate bitstream_io;

extern crate num_bigint;
use num_bigint::BigUint;

/*
 一直不喜欢排列组合类的题目，我用程序来模拟，最后只能得到6.8187，最后google找到了答案。

 20个球里出现7种颜色球的概率，
 等价于：20个球里出现至少1个红球，至少1个绿球，...，至少1个紫球的概率
 由于出现红、绿、...、紫球的概率是一样的，所以问题又等价于
 7 * {20个球里出现至少1个红球的概率}
 7 * (1 - 20个球里没有出现1个红球的概率)
 等价于：7 * (1 - 20个球里没有红球出现的所有可能组合 / 70个球里选20个球的所有可能组合）
 70个球里，除掉10个红球，还有60个其它颜色的球，从里面选20个的情况共有 C(60, 20)种
 最后结果 = 7 * (1 - C(60, 20)) / C(70, 20)
*/

fn factorial(n: u64) -> BigUint {
    let mut prod = BigUint::from(1 as u64);
    for i in 2..=n {
        prod *= BigUint::from(i as u64);
    }
    prod
}

fn combi(m: u64, n: u64) -> BigUint {
    factorial(m) / factorial(n) / factorial(m - n)
}

use num_traits::cast::ToPrimitive;
fn main() {
    let prob = 7.0 * (1.0 - combi(60, 20).to_f64().unwrap() / combi(70, 20).to_f64().unwrap());
    println!("{}", prob);

    // 后面尝试用模拟的方法，计算精度大概能到小数点后4位
    // 初始化70个球，bit0, bit1 ... bit6用来表示7种颜色
    let mut initial_balls: Vec<u32> = vec![];
    for i in 1..=10 {
        initial_balls.push((i << 8) + 1); // color 0
        initial_balls.push((i << 8) + 2); // color 1
        initial_balls.push((i << 8) + 4); // color 2
        initial_balls.push((i << 8) + 8); // color 3
        initial_balls.push((i << 8) + 16); // color 4
        initial_balls.push((i << 8) + 32); // color 5
        initial_balls.push((i << 8) + 64); // color 6
    }

    let mut rng = thread_rng();
    let total_samples = 1000_000_u64;
    let mut distinct_color_balls = 0;
    for _i in 0..total_samples {
        let mut balls = initial_balls.clone();
        let mut color_mask: u32 = 0;
        for j in 0..20 {
            let index: u32 = rng.gen_range::<u32>(0, 70 - j);
            let ball_selected = balls[index as usize]; //每一轮选出一个球
            balls[index as usize] = balls[70 - 1 - j as usize]; // 最后一个球移到前面来
            color_mask |= ball_selected;
            color_mask &= 0b_1111111;
            if color_mask == 0b_1111111 {
                break;
            }
        }
        // 统计后七位出现的二进制1的个数，就是不同颜色的球的个数
        distinct_color_balls += color_mask.count_ones() as u64;
    }
    println!("{}", (distinct_color_balls as f64) / (total_samples as f64));
}

// 两个抽样函数的写法
// let sample = seq::sample_iter(&mut rng, 0..70, 20).unwrap();
// println!("{:?}", sample);
// let sample = seq::sample_slice(&mut rng, &balls, 20);
// println!("{:?}", sample);
