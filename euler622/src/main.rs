fn main() {
    println!("{}", sum_s(8));
    println!("{}", sum_s(60));
}
// 3010983666182123972

fn sum_s(n: u32) -> u64 {
    let p = 2_u64.pow(n) - 1;
    let factors = primes::factors(p);
    // 2 ^ 64 - 1 的因子： [3, 3, 5, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321]

    let mut v = vec![];
    // 用了一种笨办法实现所有的排列组合
    // 二进制位为1表示含这个因子，0表示不包含
    for i in 1..=2_u64.pow(factors.len() as u32) {
        let s = format!("{:0width$b}", i, width = factors.len());
        let prod = s
            .chars()
            .enumerate()
            .map(|(index, x)| {
                if x.to_digit(10).unwrap() == 1 {
                    factors[index]
                } else {
                    1
                }
            })
            .fold(1_u64, |p, a| p * a as u64)
            + 1; // (x-1)肯定是(2 ^ 60 - 1)的因子，最后别忘了+1
        if !v.contains(&prod) {
            v.push(prod);
        }
    }
    //v.sort();
    //println!("{:?}",v );

    v.iter().filter(|&x| shuffle_times(*x) == n).sum::<u64>()
}

fn sum_s_v0(n: u64) -> u64 {
    let mut sum = 0_u64;
    for i in (4..999999).step_by(2) {
        let t = shuffle_times(i) as u64;
        if t == n {
            sum += i as u64;
            println!("s({}) = {}", i, t)
        }
    }
    sum
}

#[test]
fn test_shuffle_52_cards() {
    assert_eq!(8, shuffle_times(52));
}

#[test]
fn test_shuffle_86_cards() {
    assert_eq!(8, shuffle_times(86));
}

fn shuffle_times(deck_size: u64) -> u32 {
    let mut pos = 1;
    pos = shuffle_pos(deck_size, pos);
    let mut count = 1;
    while pos != 1 {
        //print!("{} ", pos);
        pos = shuffle_pos(deck_size, pos);
        count += 1;
    }
    //println!();
    count
}

fn shuffle_pos(deck_size: u64, i: u64) -> u64 {
    (i * 2) % (deck_size - 1)
    /*
    if i < deck_size / 2 {
        i * 2
    } else {
        i * 2 - deck_size + 1
    }
    */
}

fn shuffle_times_v0(deck_size: u32) -> u32 {
    let cards: Vec<u32> = (0..deck_size).collect();
    //println!("{:?}", cards);
    let mut shuffled = perfect_shuffle(&cards);
    let mut count = 1;
    while cards.to_vec() != shuffled {
        //println!("{:?}", shuffled);
        shuffled = perfect_shuffle(&shuffled);
        count += 1;
    }
    count
}

fn perfect_shuffle(cards: &[u32]) -> Vec<u32> {
    let half = cards.len() / 2;
    let mut shuffled = Vec::with_capacity(cards.len());
    for i in 0..half {
        shuffled.push(cards[i]);
        shuffled.push(cards[half + i]);
    }
    shuffled
    /*
    let c1 = &cards[0..half];
    let c2 = &cards[half..];
    let z: Vec<(_, _)> = c1.iter().zip(c2.iter()).map(|(&a, &b)| (a, b)).collect();
    z.iter().flat_map(|tup| vec![tup.0, tup.1]).collect()
    */
}
