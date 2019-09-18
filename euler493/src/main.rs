use rand::{seq, thread_rng, Rng};
extern crate bitstream_io;
//use itertools::Itertools;

fn main() {
    let mut rng = thread_rng();
    // for i in 1..100 {
    //     let sample = seq::sample_iter(&mut rng, 0..70, 20).unwrap();
    //     println!("{:?}", sample);
    // }

    let mut initial_balls = vec![];
        for i in 1..=10 {
            initial_balls.push((i << 8) + 1u32);
            initial_balls.push((i << 8) + 2);
            initial_balls.push((i << 8) + 4);
            initial_balls.push((i << 8) + 8);
            initial_balls.push((i << 8) + 16);
            initial_balls.push((i << 8) + 32);
            initial_balls.push((i << 8) + 64);
        }

    let mut sample_count: u64 = 0;
    for i in 1_u64..1000000000 {
        let mut balls = initial_balls.clone();
        let mut bits: u32 = 0;
        for  dice_count in 0..20 {
            let index: usize = rng.gen_range::<u32>(0, (balls.len() - dice_count)as u32) as usize;
            let will_remove = balls[index];
            balls[index] = balls[balls.len() - dice_count - 1];
            //balls.remove_item(&id);
            bits |= will_remove & 127;
            if bits == 127 {
                break;
            }
        }
        //println!("{:b}", a);
        sample_count += bits.count_ones() as u64;
        if i % 1000000 == 0 {
            println!("{}", (sample_count as f64) / (i as f64));
        }
    }
    //println!("{}", (sample_count as f64) / (i as f64));
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
