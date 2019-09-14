fn main() {
    let mut v: Vec<u32> = vec![];
    for a in 2..9876 {
        for b in 2..a {
            let c = a * b;
            // 关键的一条优化语句
            if c > 9876 {
                break;
            }
            let abc = a.to_string() + &b.to_string() + &c.to_string();
            if abc.len() == 9 && exists_only_once_1_to_9(&abc) {
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

fn exists_only_once_1_to_9(s: &str) -> bool {
    let mut has_digit = vec![false; 10];
    for ch in s.to_string().chars() {
        let c = ch.to_digit(10).unwrap() as usize;
        if c == 0 {
            // 不允许0的存在
            return false;
        }
        if has_digit[c] {
            return false;
        }
        has_digit[c] = true;
    }
    true
}
