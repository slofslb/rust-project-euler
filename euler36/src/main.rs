fn main() {
    let mut sum = 0;
    for n in 1..1_000_000 {
        let s10 = n.to_string();
        let s2 = format!("{:b}", n);
        //let s2 = to_radix2_string(n);
        if is_palindromic(s10) && is_palindromic(s2) {
            println!("{} {:b}", n, n);
            sum += n;
        }
    }
    println!("sum: {}", sum);
}

fn is_palindromic(s:String) -> bool {

    s == s.chars().rev().collect::<String>() 
}

fn to_radix2_string(n: u64) -> String {
    let mut d = n;
    let mut s:String = "".to_string();
    while d != 0 {
        let m = d % 2;
        s.push_str(&m.to_string());
        d /= 2;
    }
    s
}