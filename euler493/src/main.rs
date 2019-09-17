use rand::{seq, thread_rng};
extern crate bitstream_io;
use itertools::Itertools;

fn main() {
    let mut rng = thread_rng();
    // for i in 1..100 {
    //     let sample = seq::sample_iter(&mut rng, 0..70, 20).unwrap();
    //     println!("{:?}", sample);
    // }

    let mut balls = vec![];
        for i in 1..=10 {
            balls.push((i << 8) + 1u32);
            balls.push((i << 8) + 2);
            balls.push((i << 8) + 4);
            balls.push((i << 8) + 8);
            balls.push((i << 8) + 16);
            balls.push((i << 8) + 32);
            balls.push((i << 8) + 64);
        }
    

    let it = balls.iter().combinations(20);
    let mut total :u64 = 0;
    let mut count:u64 = 0;
    for sample in it {
        //println!("{:?}", sample);
        total += 1;
        let mut a = 0;
        for &ball in &sample {
            a |= ball;
        }
        a &= 127;
        //println!("{:b}", a);
        count += a.count_ones() as u64;
        if total % 10000000 == 0 {
            println!("{}", (count as f64) / (total as f64));
        }    
    }
    println!("{}", (count as f64) / (total as f64));
/*
    let mut count:u64 = 0;
    let total_count = 10000000000_u64;
    for i in 1.. {
        let sample = seq::sample_slice(&mut rng, &balls, 20);
        //println!("{:?}", sample);
        let mut a = 0;
        for ball in &sample {
            a |= *ball;
        }
        a &= 127;
        //println!("{:b}", a);
        count += a.count_ones() as u64;
        //count += distinct_colors.len();
        if i % 10000000 == 0 {
            println!("{}", (count as f64) / (i as f64));
        }
    }
    */
}
