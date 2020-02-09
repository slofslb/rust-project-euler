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
                "{:3}   {:6} {:6} {:?} vs. {:?}",
                count, v1, v2, hand1, hand2
            );
        }
    }
    println!("{}", count);
}

fn eval(hand: &[&str]) -> usize {
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
    if flush && straight && highest_value == 14 {
        return 99999;
    } else if straight && flush {
        return 99000 + highest_value;
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

fn is_straight(hand: &[&str]) -> bool {
    let mut v: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    v.sort();
    (0..4).map(|i| v[i + 1] - v[i]).all(|x| x == 1) //两两之差都为1
}

use itertools::Itertools;
fn is_flush(hand: &[&str]) -> bool {
    hand.concat()
        .split(|c| c != 'S' && c != 'H' && c != 'D' && c != 'C')
        .all_equal()
}

// 2=2, 3=3, ... J=11, Q=12, K=13, A=14
fn card_value(card: &str) -> usize {
    let all_cards = "..23456789TJQKA";
    let ch = card.chars().next().unwrap();
    all_cards.chars().position(|c| c == ch).unwrap()
}
