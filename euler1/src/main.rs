fn main() {
    let mut sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("{}", sum);

    // write in one line
    println!(
        "{}",
        (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<u32>() // 如果忘了冒号，会出现什么错误？
    );

    // fold()
    println!(
        "{}",
        (1..1000)
            .filter(|x| x % 3 == 0 || x % 5 == 0)
            .fold(0, |s, a| s + a)
    );

    println!(
        "{:?}",
        (1..1000)
            .filter(|x| x % 3 == 0 || x % 5 == 0)
            .collect::<Vec<u32>>()
    );
}
// 233168
