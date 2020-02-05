fn main() {
    assert_eq!(format_radix(43, 6, 6), "000111");

    let mut count1 = vec![0; 37];
    for n in 0..6_u32.pow(6) {
        let m = format_radix(n, 6, 6);
        //println!("{:?}", m);
        let s = m.chars().map(|c| c.to_digit(10).unwrap() + 1).sum::<u32>();
        count1[s as usize] += 1; 
    }
    let total = count1.iter().sum::<u32>();
    println!("{:?}", 6*6*6*6*6*6);
    println!("{:?}", total);
    println!("{:?}", count1);

    let mut count2 = vec![0; 37];
    for n in 0..4_u32.pow(9) {
        let m = format_radix(n, 4, 9);
        //println!("{:?}", m);
        let s = m.chars().map(|c| c.to_digit(10).unwrap() + 1).sum::<u32>();
        count2[s as usize] += 1; 
    }
    let total = count2.iter().sum::<u32>();
    println!("{:?}", total);
    println!("{:?}", count2);

    let mut prob = 0_u64;
    for i in 6..=36 {
        let prob1 = count1[i] as u64;
        let prob2 = count2[i+1..=36].iter().sum::<u32>() as u64;
        prob += prob1 * prob2;
        println!("{} {}", prob1, prob2);
    }
    println!("{}", prob);
    println!("{}", (prob as f64) / 6.0_f64.powf(6.0) / 4.0_f64.powf(9.0));
}

fn format_radix(mut x: u32, radix: u32, length:u32) -> String {
    let mut result = vec![];

    for _i in 0..length {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        // if x == 0 {
        //     break;
        // }
    }
    result.into_iter().rev().collect()
}
