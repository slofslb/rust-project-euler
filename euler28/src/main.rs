fn main() {
    let mut sum = 1;
    for i in 1..=500 {
        let upperright = (i*2+1) * (i*2+1);
        sum += upperright;
        sum += upperright - (i*2) * 1;
        sum += upperright - (i*2) * 2;
        sum += upperright - (i*2) * 3;
    }
    println!("{}", sum);
}
// 669171001
