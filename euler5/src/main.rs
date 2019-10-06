fn main() {
    'outer: for x in (100..).step_by(2) {
        for f in (2..=20).rev() {
            if x % f != 0 {
                break;
            }
            if f == 2 {
                println!("{}", x);
                break 'outer;
            }
        }
    }
}
// 232792560
