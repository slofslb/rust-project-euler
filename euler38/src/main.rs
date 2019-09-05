fn main() {
    let mut max = "".to_string();
    for a in 1..=9876 {
        let mut s = String::from("");
        for n in 1..=9 {
            let prod = a * n;    
            s.push_str(&prod.to_string());
            if !contains_only_once_1to9(&s) {break;}
            if s.len() == 9 && s > max {
                println!("{} {} {}", a, n, s);
                max = s.clone();
            }
        }
    }
}
// 9327 2 932718654

fn contains_only_once_1to9(s: &str) -> bool {
    let mut digits : Vec<bool> = vec![false; 10];
    for ch in s.to_string().chars() {
        let c = ch.to_digit(10).unwrap() as usize;
        if c == 0 {return false;}
        if digits[c] {return false;}
        digits[c] = true;
    }
    true 
}