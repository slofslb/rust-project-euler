fn main() {
    let mut v: Vec<u32> = vec![];
    for a in 2..9000 {
        if !is_only_once(&a.to_string()) {continue;}
        for b in (a+1)..9000 {
            let c = a * b;    
            if c > 9876 {break;} //这是关键的一条优化，其它的作用都不大
            if !is_only_once(&b.to_string()) {continue;}
            let ab = a.to_string() + &b.to_string();
            if !is_only_once(&ab) {continue;}
            let abc = ab + &c.to_string();
            if abc.len() == 9 && is_only_once(&abc) {
                println!("{} x {} = {}", a, b, c);
                if !v.contains(&c) {
                    v.push(c);
                }
            }
        }
    }
    println!("{}", v.iter().sum::<u32>());
}
// 45228
// 优化：1）排序太慢，修改判断逻辑，
//2）不会超过5000，
//3）a < b，
//4）乘积大于9876时，内循环不用再做了，后面的数只会更大


fn is_only_once(s: &str) -> bool {
    let mut digits : Vec<bool> = vec![false; 10];
    for ch in s.to_string().chars() {
        let c = ch.to_digit(10).unwrap() as usize;
        if c == 0 {return false;}
        if digits[c] {return false;}
        digits[c] = true;
    }
    true 
}
/*
fn is_only_once(s: &str) -> bool {
    if s.contains('0') {return false;}
    let mut ch = s.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let len1 = ch.len();
    //println!("{:?}", ch);
    ch.sort();
    ch.dedup();
    //println!("{:?}", ch);
    let len2 = ch.len();
    len1 == len2
}
*/