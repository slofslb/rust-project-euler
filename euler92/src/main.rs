fn main() {
    let mut count = 0;
    for i in 1..10_000_000 {
        if square_chain_arrive(i) == 89 {
            count += 1;
        }
    }
    println!("{}", count);
}
// 8581146

fn square_chain_arrive(n: u64) -> u64 {
    let mut x = n;
    while x != 1 && x != 89 {
        x = square_sum(x);
    }
    x
}

fn square_sum(n: u64) -> u64 {
    let mut m = n;
    let mut s = 0;
    while m != 0 {
        s += (m % 10) * (m % 10);
        m /= 10;
    }
    s
    // n.to_string()
    //     .chars()
    //     .map(|x| x.to_digit(10).unwrap().pow(2) as u64)
    //     .sum::<u64>()
}
