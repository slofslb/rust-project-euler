fn main() {
    for x in 100000..=999999 {
        if is_permuted(x) {
            println!("{}", x);
            break;
        }
    }
}

fn is_permuted(x: u32) -> bool {
    // 拆成6个数字
    let vx: Vec<u32> = x
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    for i in 2..=6 {
        let m = i * x;
        let vm: Vec<u32> = m
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        for e in vm {
            if !vx.contains(&e) {
                return false;
            }
        }
    }
    return true;
}
