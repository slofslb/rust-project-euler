fn main() {
    let result = summation_ways(100, 99);
    println!("{}", result);
}
// 190569291 

fn summation_ways(s: u32, limit: u32) -> u32 {
    if limit <= 1 {
        return 1;
    }
    let mut count = 0;
    for i in 1..=limit {
        count += summation_ways(s - i, i.min(s - i));
    }
    //println!("{} {} {}", s, limit, count);
    count
}
