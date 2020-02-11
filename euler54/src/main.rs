#[derive(Debug)]
enum SuitType {
    HighCard(usize), // 单牌
    //对子
    OnePair {
        value_pair: usize,
        max_remain: usize, // 除了对子之外，剩下最大的牌点
    },
    //两对
    TwoPairs {
        high_pair: usize,  // 最大的一对
        low_pair: usize,   // 最小的一对
        max_remain: usize, // 除了两对之外，剩下最大的牌点
    },
    KindThree(usize), // 三条
    Straight(usize),  // 顺子
    Flush(usize),     // 同花
    //葫芦，即三条带一个对子
    FullHouse {
        value_kind_three: usize,
        value_pair: usize,
    },
    KindFour(usize),      // 四条
    StraightFlush(usize), // 同花顺
    RoyalFlush,           // 同花大顺
}

fn main() {
    let data = std::fs::read_to_string("poker.txt").expect("打开文件出错");
    let data2 = data.replace("\r\n", "\n");
    let lines = data2.trim().split('\n');
    let mut count = 0;
    for line in lines {
        let cards: Vec<_> = line.split(' ').collect();
        let (hand1, hand2) = (&cards[..5], &cards[5..]);
        let (v1, v2) = (eval(hand1), eval(hand2));
        if v1 > v2 {
            count += 1;
            println!(
                "{:3}   {:8} {:8} {:?} vs. {:?}",
                count, v1, v2, hand1, hand2
            );
        }
    }
    println!("player1 > player2: {}", count);
}

fn eval(hand: &[&str]) -> usize {
    let t = suit_type(hand);
    match t {
        SuitType::RoyalFlush => 9_999_999,
        SuitType::StraightFlush(a) => 6_000_000 + a,
        SuitType::KindFour(a) => 5_000_000 + a,
        SuitType::FullHouse {
            value_kind_three: a,
            value_pair: b,
        } => 4_000_000 + a * 100 + b,
        SuitType::Flush(a) => 3_000_000 + a,
        SuitType::Straight(a) => 2_000_000 + a,
        SuitType::KindThree(a) => 1_000_000 + a,
        SuitType::TwoPairs {
            high_pair: a,
            low_pair: b,
            max_remain: c,
        } => 10000 * a + 100 * b + c,
        SuitType::OnePair {
            value_pair: a,
            max_remain: b,
        } => 100 * a + b,
        SuitType::HighCard(a) => a,
        // TODO: 实际上程序还有隐藏的BUG，当HighCard时，仅保存最大的牌点不行
        // 例如 ：A9753和A9752，仅比较最大的牌是不行的，还得依次比较最大的牌点才能出来结果
    }
}

fn is_straight(hand: &[&str]) -> bool {
    let mut v: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    v.sort();
    (0..4).all(|i| v[i + 1] - v[i] == 1) //两两之差都为1
}

// 一手牌里出现某个牌点i的个数
fn count_cards(hand: &[&str], i: usize) -> usize {
    hand.iter()
        .map(|x| card_value(x))
        .filter(|&x| x == i)
        .count()
}

use itertools::Itertools;
fn is_flush(hand: &[&str]) -> bool {
    //println!("{:?}", hand.iter().map(|x| &x[1..]).collect::<Vec<&str>>());
    hand.iter().map(|card| &card[1..]).all_equal()
}

// 2=2, 3=3, ... J=11, Q=12, K=13, A=14
fn card_value(card: &str) -> usize {
    let first_char = card.chars().next().unwrap();
    "..23456789TJQKA"
        .chars()
        .position(|c| c == first_char)
        .unwrap()
}

// 统计各种牌点出现的次数，例如：[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0]
// 表示五张牌里有3张10，1张J(11点)，1张Q(12点)
fn cards_stat(hand: &[&str]) -> Vec<usize> {
    // 对不同的牌计数
    let mut count_cards = vec![0; 15]; // 最大A，值是14
    let values: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    for v in values {
        count_cards[v] += 1;
    }
    count_cards
}

fn suit_type(hand: &[&str]) -> SuitType {
    // 最大的牌点
    let max_value = hand.iter().map(|x| card_value(x)).max().unwrap();

    if is_flush(hand) && is_straight(hand) {
        if max_value == 14 {
            return SuitType::RoyalFlush;
        } else {
            return SuitType::StraightFlush(max_value);
        }
    } else if is_flush(hand) {
        return SuitType::Flush(max_value);
    } else if is_straight(hand) {
        return SuitType::Straight(max_value);
    }

    //let stat :Vec<usize> = (0..=14).map(|v| count_cards(hand, v)).collect();
    let stat = cards_stat(hand);
    

    if let Some(a) = stat.iter().position(|&x| x == 4) {
        return SuitType::KindFour(a);
    } else if let Some(a) = stat.iter().position(|&x| x == 3) {
        // 有三条出现
        if let Some(b) = stat.iter().position(|&x| x == 2) {
            //如果还有一对，就是葫芦
            return SuitType::FullHouse {
                value_kind_three: a,
                value_pair: b,
            };
        }
        return SuitType::KindThree(a);
    } else if let Some(a) = stat.iter().rposition(|&x| x == 2) {
        // 寻找最大的对子
        let b = stat.iter().position(|&x| x == 2).unwrap();
        let max_hight_card = stat.iter().rposition(|&x| x == 1).unwrap(); // 单牌的最大值
        if a != b {
            //如果还有一对
            return SuitType::TwoPairs {
                high_pair: a,
                low_pair: b,
                max_remain: max_hight_card,
            };
        }
        return SuitType::OnePair {
            value_pair: a,
            max_remain: max_hight_card,
        };
    }

    SuitType::HighCard(max_value)
}

fn eval_old(hand: &[&str]) -> usize {
    // 对不同的牌计数
    let mut count_cards = vec![0; 15]; // 最大A，值是14
    let values: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    let mut highest_value = 0;
    for v in values {
        count_cards[v] += 1;
        if v > highest_value {
            highest_value = v;
        }
    }

    // 记录对子、三条、四条
    let mut kind_four = 0; //四条，四个牌点相同
    let mut kind_three = 0; //三条，三个牌点相同
    let mut pair: Vec<usize> = vec![]; //可能有2对
    let mut remain: Vec<usize> = vec![];
    for (card_value, &count) in count_cards.iter().enumerate() {
        if count == 4 {
            kind_four = card_value;
        } else if count == 3 {
            kind_three = card_value;
        } else if count == 2 {
            pair.push(card_value);
        } else {
            remain.push(card_value);
        }
    }

    let flush = is_flush(hand);
    let straight = is_straight(hand);
    if straight && flush {
        return 99900 + highest_value;
    } else if kind_four > 0 {
        return 90000 + kind_four;
    } else if kind_three > 0 && !pair.is_empty() {
        return 80000 + kind_three * 100 + pair[0];
    } else if flush {
        return 70000 + highest_value;
    } else if straight {
        return 60000 + highest_value;
    } else if kind_three > 0 {
        return 50000 + kind_three;
    } else if pair.len() == 2 {
        pair.sort();
        return pair[1] * 2000 + pair[0] * 100 + remain[0];
    } else if pair.len() == 1 {
        return pair[0] * 100 + remain[0].max(remain[1]);
    }
    highest_value
}
