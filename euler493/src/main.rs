use rand::{seq, thread_rng};
fn main() {
    let mut rng = thread_rng();
    // for i in 1..100 {
    //     let sample = seq::sample_iter(&mut rng, 0..70, 20).unwrap();
    //     println!("{:?}", sample);
    // }

    let mut balls = vec![];
    for color in 0..7 {
        for i in 0..10 {
            balls.push((color, i));
        }
    }

    let mut count = 0;
    let total_count = 1000000000;
    for i in 1..=total_count {
        let sample = seq::sample_slice(&mut rng, &balls, 20);
        let mut distinct_colors = vec![];
        for (color, _) in &sample {
            if !distinct_colors.contains(&color) {
                distinct_colors.push(color);
                if distinct_colors.len() == 7 {break;}
            }
        }
        //println!("{:?} \n{:?}", sample, distinct_colors.len() );
        count += distinct_colors.len();
        if i % 1000000 == 0 {
            println!("{}", (count as f64) / (i as f64));
        }
    }
}
