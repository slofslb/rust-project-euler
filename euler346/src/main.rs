fn main() {
    // 最早的尝试，打印了一张表
    for n in 1..14 {
        print!("n={}", n);
        for b in 2_u64..12 {
            let t = (b.pow(n) - 1) / (b - 1);
            print!("{:15}", t);
        }
        println!();
    }

    println!("{}", repunit(10_u128.pow(12)));
}

// 只有两个数会重复出现，31和8191
// https://en.wikipedia.org/wiki/Goormaghtigh_conjecture
fn cal_goormaghtigh_conjecture_numbers() -> Vec<u128> {
    let mut cache = vec![0_u8; 1_000_000];
    for b in 2_u128.. {
        let n =
            (((cache.len() as f64) * (b as f64 - 1.0) + 1.0).log10() / (b as f64).log10()) as u32;
        if n == 2 {
            break;
        }
        for i in 3..=n {
            let x = ((b.pow(i) - 1) / (b - 1)) as usize;
            if cache[x] >= 1 {
                println!("base:{} repeat:{} {}", b, i, x);
            }
            cache[x as usize] += 1;
        }
    }
    let goormaghtigh_conjecture_numbers: Vec<_> = cache
        .iter()
        .enumerate()
        .filter(|(_, &x)| x > 1)
        .map(|(i, _)| i as u128)
        .collect();
    goormaghtigh_conjecture_numbers
}

fn repunit(limit: u128) -> u128 {
    let magic_numbers = cal_goormaghtigh_conjecture_numbers();

    let mut total: u128 = 0;
    for b in 2_u128.. {
        let n = (((limit as f64) * (b as f64 - 1.0) + 1.0).log10() / (b as f64).log10()) as u32;
        if n == 2 {
            break;
        }
        let sum = (b * (b.pow(n) - 1) / (b - 1) - (n as u128)) / (b - 1);
        let sum = sum - (b + 1) - 1;
        total += sum;
        //println!("b:{} n:{} {} {}", b, n, sum, total);
    }
    if limit > magic_numbers[0] {
        total -= magic_numbers[0];
    }
    if limit > magic_numbers[1] {
        total -= magic_numbers[1];
    }
    total + 1
}

#[test]
fn test_50() {
    assert_eq!(repunit(50), 171);
}

#[test]
#[ignore]
fn test_1000() {
    assert_eq!(repunit(1000), 15864);
}
