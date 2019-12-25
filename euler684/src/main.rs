fn main() {
    let mut sum = 0;
    for n in 1..=20 {
        let b = n / 9;
        let a = n % 9;
        let temp = (a+1) * 10_u32.pow(b) - 1;
        println!("n:{} temp: {}", n, temp);
        sum += temp;
    }
    println!("{}", sum);
}
