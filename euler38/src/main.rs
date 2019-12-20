// 第32题相似
fn main() {
    let mut max = "".to_string();
    for n in 1..=9999 {
        let mut s = String::from("");
<<<<<<< HEAD
        for i in 1..=9 {
            let prod = n * i;    
=======
        for n in 1..=9 {
            let prod = a * n;
>>>>>>> d9774854d5af171a29efad92c087213926be9ab9
            s.push_str(&prod.to_string());
            if !exists_only_once_1_to_9(&s) {
                break;
            }
            if s.len() == 9 && s > max {
<<<<<<< HEAD
                println!("{} {} {}", n, i, s);
=======
                println!("{} x {:?} = {}", a, [1..n], s);
>>>>>>> d9774854d5af171a29efad92c087213926be9ab9
                max = s.clone();
            }
        }
    }
}
// 9327 2 932718654

fn exists_only_once_1_to_9(s: &str) -> bool {
    let mut has_digit: Vec<bool> = vec![false; 10];
    for ch in s.to_string().chars() {
        let c = ch.to_digit(10).unwrap() as usize;
        if c == 0 {
            return false;
        }
        if has_digit[c] {
            return false;
        }
        has_digit[c] = true;
    }
    true
}
