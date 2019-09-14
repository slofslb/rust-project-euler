fn main() {
    // let mut fac = vec![1; 10];
    // for i in 0..=9 {
    //     fac[i] = factorial(i as u32);
    // }
    let fac: Vec<u32> = (0..10).map(|x| factorial(x)).collect();
    println!("{:?}", fac);

    let mut sum = 0;
    for n in 3..999999 {
        // 各位数字的阶乘之和
        // let mut sum_fac = 0;
        // for digit in n.to_string().chars().map(|x| x.to_digit(10).unwrap()) {
        //     sum_fac += fac[digit as usize];
        // }
        let sum_fac: u32 = n
            .to_string()
            .chars()
            .map(|x| fac[x.to_digit(10).unwrap() as usize])
            .sum();
        if n == sum_fac {
            println!("{}! = {}", n, n);
            sum += n;
        }
    }
    println!("{}", sum);
}

fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
