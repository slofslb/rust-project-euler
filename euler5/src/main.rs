fn main() {
    let mut x = 100;
    'outer: loop {
        for f in (2..=20).rev() {
            if x % f != 0 {break;}
            if f == 2 {
                println!("{}", x);
                break 'outer;
            }
        }
        x += 2;
    }
}
// 232792560