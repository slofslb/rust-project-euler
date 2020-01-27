fn main() {
    // 借鉴第34题
    let fac_0_to_9: Vec<u32> = (0..10).map(|x| factorial(x)).collect();

    println!("{}", sum_fac(&fac_0_to_9, 145));
    println!("{}", sum_fac(&fac_0_to_9, 169));
}

fn sum_fac(fac_0_to_9: &Vec<u32>, n: u32) -> u32{
    n.to_string()
     .chars()
     .map(|x| fac_0_to_9[x.to_digit(10).unwrap() as usize])
     .sum()
}

fn factorial(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n - 1)
}
