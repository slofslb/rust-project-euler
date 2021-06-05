use rand::{seq, thread_rng, Rng};
//use itertools::Itertools;

fn main() {
    let mut rng = thread_rng();
    let total_samples = 6;
    let mut sum = 0;
    let balls20 = seq::sample_iter(&mut rng, 1..=6, 3).unwrap();
    println!("{:?}", balls20);
}