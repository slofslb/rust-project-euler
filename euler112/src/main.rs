fn main() {
    assert!(!is_bouncy_num(13468));
    assert!(!is_bouncy_num(66420));
    assert!(is_bouncy_num(155349));
    let mut count = 0;
    for n in 1.. {
        if is_bouncy_num(n) {
            count += 1;
            if count * 100 == n * 50 {
                println!("50% n={}", n);
            }
            if count * 100 == n * 90 {
                println!("90% n={}", n);
            }
            if count * 100 == n * 99 {
                println!("99% n={}", n);
                break;
            }
        }
    }
}

fn is_bouncy_num(n: u64) -> bool {
    if n < 100 {
        return false;
    }
    let digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i8)
        .collect::<Vec<i8>>();
    let mut dir = 0;
    for i in 0..digits.len() - 1 {
        let sign = (digits[i + 1] - digits[i]).signum();
        if sign == 0 {
            continue;
        }
        if dir == 0 {
            dir = sign;
            continue;
        }
        if sign != dir {
            return true;
        }
    }
    false
}
