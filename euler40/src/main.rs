fn main() {
    let max_digits = 1_000_001;
    let mut digits: Vec<usize> = vec![0; max_digits];
    let mut pos = 1;
    'a: for i in 1.. {
        for ch in i.to_string().chars() {
            let d = ch.to_digit(10).unwrap() as usize;
            if pos >= max_digits {
                break 'a;
            }
            digits[pos] = d;
            pos += 1;
        }
    }

    println!(
        "{}", digits[1]
            * digits[10]
            * digits[100]
            * digits[1000]
            * digits[10000]
            * digits[100000]
            * digits[1000000]
    );

    // 用map()和fold()的写法
    let d: Vec<usize> = (0..=6).map(|x| digits[10_usize.pow(x)]).collect();
    println!("{:?}", d);

    let p: usize = (0..=6)
        .map(|x| digits[10_usize.pow(x)])
        .fold(1, |x, a| x * a);
    println!("{}", p);
}
