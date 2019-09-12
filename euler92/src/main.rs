fn main() {
    let mut count = 0;
    for i in 1..10_000_000 {
        if square_chain_arrive(i) == 89 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn square_chain_arrive(n: u32) -> u32 {
    let mut x = n;
    while x != 1 && x != 89 {
        x = square_sum(x);
    }
    x
}

fn square_sum(n: u32) -> u32 {
    n.to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap().pow(2))
        .sum::<u32>()
}