fn main() {
    let start = 1020304050607080900.0_f64.sqrt() as u64;
    let end = 1929394959697989990.0_f64.sqrt() as u64;
    for i in start..=end {
        if i % 10 != 0 { //最后1位必须是0
            continue;
        } 
        let m = i * i;
        let chars: Vec<u32> = m
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        for j in 0..9 {
            if chars[j * 2] != (j + 1) as u32 {
                break;
            }
            if j == 8 {
                println!("{} {}", i, m);
                return;
            }
        }
    }
}
