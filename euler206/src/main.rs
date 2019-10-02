fn main() {
    let start = (1020304050607080900.0_f64.sqrt() as u64) / 10 * 10;
    let end = 1929394959697989990.0_f64.sqrt() as u64;
    for i in (start..=end).step_by(10) {
        let m = i * i;
        if meet_cond(m) {
            println!("{} {}", i, m);
            break;
        }
    }
}

fn meet_cond(m: u64) -> bool {
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
