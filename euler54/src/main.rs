fn main() {
    let data = std::fs::read_to_string("poker.txt").expect("打开文件出错");
    let lines = data.trim().split('\n');
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

// 1)读文件
// 2)取出两手牌
// 3）比较最大的牌张
// 4) 顺子，同花，同花顺，同花大顺
// 5) 四条

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
        return 90000 + highest_value;
    } else if kind_four > 0 {
        return 80000 + kind_four;
    } else if kind_three > 0 && pair.len() > 0 {
        return 70000 + kind_three * 100 + pair[0];
    } else if flush {
        return 60000 + highest_value;
    } else if straight {
        return 50000 + highest_value;
    } else if kind_three > 0 {
        return 40000 + kind_three;
    } else if pair.len() == 2 {
        pair.sort();
        return 30000 + pair[1] * 500 + pair[0] * 50 + remain[0];
    } else if pair.len() == 1 {
        return 20000 + pair[0] * 100 + remain[0].max(remain[1]);
    }
    highest_value
}

// 是顺子？
fn is_straight(hand: &[&str]) -> bool {
    let mut v: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    v.sort();
    (0..4).map(|i| v[i + 1] - v[i]).all(|x| x == 1) //两两之差都为1

    // let diff1 = v[1] - v[0];
    // let diff2 = v[2] - v[1];
    // let diff3 = v[3] - v[2];
    // let diff4 = v[4] - v[3];
    // diff1 == 1 && diff1 == diff2 && diff2 == diff3 && diff3 == diff4'
}

use itertools::Itertools;
// 是同花？
fn is_flush(hand: &[&str]) -> bool {
    hand.concat()
        .split(|c| c != 'S' && c != 'H' && c != 'D' && c != 'C')
        .all_equal()
    /*
    let hand_str = hand.concat().to_string();
    let first_suit = hand_str.chars().nth(1).unwrap();
    for i in 3..=9 {
        let suit = hand_str.chars().nth(i).unwrap();
        if first_suit != suit {
            return false;
        }
    }
    true
    */
}

// 2为2点，3为3点，J为11，Q为12，K为13，A为14
fn card_value(card: &str) -> usize {
    let all_cards = "..23456789TJQKA";
    let ch = card.chars().next().unwrap();
    all_cards.chars().position(|c| c == ch).unwrap()
}
