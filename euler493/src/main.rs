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
   let sample = seq::sample_slice(&mut rng, &balls, 20);
   let mut distinct_colors = vec![];
   for (color, _) in &sample {
       if !distinct_colors.contains(&color) {
           distinct_colors.push(color)
       }
   }
   println!("{:?} \n{:?}", sample, distinct_colors );
}
