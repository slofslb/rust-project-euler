fn main() {
    let start = (1020304050607080900.0_f64.sqrt() as u64) / 10 * 10;
    let end = 1929394959697989990.0_f64.sqrt() as u64;
    for i in (start..=end).step_by(10) {
        let m = i * i;
        if meet_cond(m) {
            println!("{} ^ 2 = {}", i, m);
            break;
        }
    }
}
// 1389019170

// 1_2_3_4_5_6_7_8_9_0
fn meet_cond(m: u64) -> bool {
    let mut x = m / 100;
    let mut digit = 9;
    while x != 0 {
        if x % 10 != digit {
            return false;
        }
        x /= 100;
        digit -= 1;
    }
    true
}

fn meet_cond2(m: u64) -> bool {
    let chars: Vec<u32> = m
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for j in 0..9 {
        if chars[j * 2] != (j + 1) as u32 {
            return false;
        }
    }
    true
}
