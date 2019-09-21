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

fn is_palindromic(s: String) -> bool {
    let mut c1 = s.chars();
    let mut c2 = s.chars().rev();
    for _i in 0..s.len() / 2 {
        if c1.next().unwrap() != c2.next().unwrap() {
            return false;
        }
    }
    true
    //    s == s.chars().rev().collect::<String>()
}

/*
// 重复发明轮子
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
*/
