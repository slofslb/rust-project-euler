use rand::{seq, thread_rng, Rng};
extern crate bitstream_io;
//use itertools::Itertools;

fn main() {
    let mut rng = thread_rng();
    // for i in 1..100 {
    //     let sample = seq::sample_iter(&mut rng, 0..70, 20).unwrap();
    //     println!("{:?}", sample);
    // }

    let mut sample_count: u64 = 0;
    for i in 1_u64..1000000000 {
        let mut balls = vec![10; 7];
        let mut bits: u32 = 0;
        for  _dice_count in 0..20 {
            let mut id: usize = (rng.gen::<u32>() % 7) as usize;
            while balls[id] == 0 {
                id = (id + 1) % 7;
            }
            balls[id] -= 1;
            bits |= 1 << id;
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
