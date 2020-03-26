use itertools::Itertools;

fn main() {
    for n in 10..100 {
        for pos in 0..=1 {
            let prime_family = gen_prime_family(n, pos);
            if prime_family.len() == 6 {
                println!("seed: {} {:?}", n, prime_family);
                break;
            }
        }
    }

    let start = 100000;
    let end = 999999;

    for n in start..end {
        if !primes::is_prime(n) {
            continue;
        }
        for position_replaced in 2..end.to_string().len() - 1 {
            let pos_combinations = (0..end.to_string().len()).combinations(position_replaced);
            for pos in pos_combinations {
                let mut prime_family = vec![];
                for digit in 0..=9 {
                    let mut n_generated = n;
                    for ip in &pos {
                        n_generated = replace(n_generated, *ip, digit);
                    }

                    if primes::is_prime(n_generated)
                        && !prime_family.contains(&n_generated)
                        && n_generated.to_string().len() == end.to_string().len()
                    {
                        prime_family.push(n_generated);
                    }
                }
                if prime_family.len() >= 8 {
                    println!("seed: {} {:?} {:?}", n, pos, prime_family);
                    println!("{}", prime_family[0]);
                    return;
                }
            }
        }
    }
}
// seed: 120383 [1, 3, 5] [121313, 222323, 323333, 424343, 525353, 626363, 828383, 929393]
// 121313

static POWER: &[u64] = &[
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
];

// 把整数n的某位换成新数字new_digit
// pos为0表示个位，1表示十位，2表示百位
fn replace(n: u64, pos: usize, new_digit: u64) -> u64 {
    let d = n / 10_u64.pow(pos as u32) % 10;
    n - d * 10_u64.pow(pos as u32) + new_digit * 10_u64.pow(pos as u32)
}

fn remove(n: u64, pos: usize) -> (u64, u64) {
    let d = n / POWER[pos] % 10;
    (n - d * POWER[pos], d)
}

fn gen_prime_family(n: u64, pos: usize) -> Vec<u64> {
    let mut prime_family = vec![];
    for digit in 0..=9 {
        let generated = replace(n, pos, digit);
        if primes::is_prime(generated)
            && !prime_family.contains(&generated)
            && generated.to_string().len() == n.to_string().len()
        {
            prime_family.push(generated);
        }
    }
    prime_family
}
