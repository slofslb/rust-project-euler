use itertools::Itertools;

fn main() {
    let start = 100000;
    let end = 999999;

    let max_number_to_check = end + 1;
    let mut prime_mask = vec![true; max_number_to_check];
    fill_prime_mask(&mut prime_mask);

    for n in start..end {
        if !prime_mask[n] {
            continue;
        }
        for position_replaced in 2..end.to_string().len() - 1 {
            let pos_combinations = (0..end.to_string().len()).combinations(position_replaced);
            for pos in pos_combinations {
                let mut prime_family = vec![];
                for digit in 0..=9 {
                    let mut n_replaced = n;
                    for ip in &pos {
                        n_replaced = replace(n_replaced, *ip, digit);
                    }

                    if prime_mask[n_replaced]
                        && !prime_family.contains(&n_replaced)
                        && n_replaced.to_string().len() == end.to_string().len()
                    {
                        prime_family.push(n_replaced);
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

static POWER: &[usize] = &[
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

fn replace(n: usize, pos: usize, new_digit: usize) -> usize {
    let d = n / POWER[pos] % 10;
    n - d * POWER[pos] + new_digit * POWER[pos]
}

fn remove(n: usize, pos: usize) -> (usize, usize) {
    let d = n / POWER[pos] % 10;
    (n - d * POWER[pos], d)
}

fn fill_prime_mask(prime_mask: &mut [bool]) {
    prime_mask[0] = false;
    prime_mask[1] = false;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..prime_mask.len() {
        if prime_mask[p] {
            let mut i = 2 * p;
            while i < prime_mask.len() {
                prime_mask[i] = false;
                i += p;
            }
        }
    }
}
