fn main() {
    let mut sum = 0;
    for n in 1..1_000_000 {
        let s10 = n.to_string();
        let s2 = to_str_radix2(n);
        if s10 == s10.chars().rev().collect::<String>() 
            && s2 == s2.chars().rev().collect::<String>() {
            println!("{}", n);
            sum += n;
        }
    }
    println!("sum: {}", sum);
}

fn to_str_radix2(n: u32) -> String {
    let mut d = n;
    let mut s:String = "".to_string();
    while d != 0 {
        let m = d % 2;
        s.push_str(&m.to_string());
        d /= 2;
    }
    s
}