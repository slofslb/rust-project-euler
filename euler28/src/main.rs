fn main() {
    let mut sum = 1;
    for i in (3..=1001).step_by(2) {
        let upperright = i * i;
        sum += upperright;
        sum += upperright - (i - 1) * 1;
        sum += upperright - (i - 1) * 2;
        sum += upperright - (i - 1) * 3;
    }
    println!("{}", sum);
}
// 669171001
