use itertools::Itertools;
use rand::{seq, thread_rng};

fn main() {
    // 7.0 * (1.0 - c(60, 20) / c(70, 20))
    let mut prob = 1.0;
    for i in 41..=50 {
        prob *= (i as f64) / ((i + 20) as f64);
    }
    println!("{:.9}", (1.0 - prob) * 7.0);

    // 模拟算法，只能得到粗略答案
    simulate();
}

fn simulate() {
    let mut rng = thread_rng();
    let total_samples = 1_000_000_u64;
    let mut sum = 0;
    //let all_balls :Vec<u32>= (0..70).collect();
    for _i in 0..total_samples {
        let balls20 = seq::sample_iter(&mut rng, 0..70, 20).unwrap();
        let distinct_colors = balls20
            .into_iter()
            .map(|x| x / 10)
            .unique()
            .collect::<Vec<u32>>();
        //println!("{:?}", distinct_colors);
        sum += distinct_colors.len();
    }
    println!("{}", (sum as f64) / (total_samples as f64));
}

/*
use num_traits::cast::ToPrimitive;
extern crate bitstream_io;

    // 后面尝试用模拟的方法，计算精度大概能到小数点后4位
    // 初始化70个球，bit0, bit1 ... bit6用来表示7种颜色
    let mut rng = thread_rng();
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
*/
