fn main() {
    let data = std::fs::read_to_string("poker.txt").expect("打开文件出错");
    let lines = data.trim().split('\n');
    let mut count = 0;
    for line in lines {
        let cards: Vec<_> = line.split(' ').collect();
        let hand1 = &cards[..5];
        let hand2 = &cards[5..];

        if is_flush(hand2) {
            print!("flush !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        }

        /*
                print!("{:?}  ", highest_card(hand1));
                print!("{:?}  ", is_straight(hand1));
                print!("{:?}  ", is_straight_flush(hand1));
                println!("{:?}  ", is_royal_flush(hand1));
                println!("four kind {:?}  ", four_of_a_kind(hand1));
                println!("three kind {:?}  ", three_of_a_kind(hand1));
        */
        let p1 = pairs(hand1);
        let p2 = pairs(hand2);
        let v1 = eval(hand1);
        let v2 = eval(hand2);
        //if p1.len() == 1 && p2.len() == 2 {
        //if v1 > 20000  || v2 > 20000 {
        if three_of_a_kind(hand1) > 0 || three_of_a_kind(hand2) > 0 {
            print!("player 1: {:?}  ", hand1);
            println!("player 2: {:?}", hand2);
            println!("pairs1 {:?}  pairs2: {:?}", pairs(hand1), pairs(hand2));
            println!("{}  {}\n", v1, v2);
        }

        if v1 > v2 {
            count += 1;
        }
    }
    println!("{}", count);
}

// 1)读文件
// 2)取出两手牌
// 3）比较最大的牌张
// 4) 顺子，同花，同花顺，同花大顺
// 5) 四条

fn eval2(hand: &[&str]) -> usize {
    let mut count_cards = vec![0; 15]; // 最大A，值是14
    let values: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    for v in values {
        count_cards[v] += 1;
    }

    let mut kind_four = 0;
    let mut kind_three = 0;
    let mut pair: Vec<usize> = vec![];
    let mut remain: Vec<usize> = vec![];

    for (card_value, count) in count_cards.iter().enumerate() {
        if *count == 4 {
            kind_four = card_value;
        } else if *count == 3 {
            kind_three = card_value;
        } else if *count == 2 {
            pair.push(card_value);
        } else {
            remain.push(card_value);
        }
    }
    0
}

fn eval(hand: &[&str]) -> usize {
    if is_royal_flush(hand) {
        return 99999;
    } else if is_straight_flush(hand) {
        return 90000 + highest_card(hand);
    } else if four_of_a_kind(hand) > 0 {
        return 80000 + four_of_a_kind(hand);
    } else if is_flush(hand) {
        return 60000 + highest_card(hand);
    } else if is_straight(hand) {
        return 50000 + highest_card(hand);
    } else if three_of_a_kind(hand) > 0 {
        let p = pairs(hand);
        if p.len() > 0 {
            return 75000 + three_of_a_kind(hand) * 100 + p[0]; // full house
        } else {
            return 40000 + highest_card(hand);
        }
    } else {
        let mut p = pairs(hand);
        if p.len() == 2 {
            p.sort();
            return 20000 + p[1] * 500 + p[0] * 100 + highest_card(hand);
        } else if p.len() == 1 {
            return 10000 + p[0] * 100 + highest_card(hand);
        }
    }
    highest_card(hand)
}

fn pairs(hand: &[&str]) -> Vec<usize> {
    let mut count_cards = vec![0; 15]; // 最大A，值是14
    let values: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    for v in values {
        count_cards[v] += 1;
    }
    let mut pair: Vec<usize> = vec![];

    for (card_value, count) in count_cards.iter().enumerate() {
        if *count == 2 {
            pair.push(card_value);
        }
    }
    pair
}

// 是否有三条的存在，如果有，返回其牌张的大小
// 如果不存在，返回0
fn three_of_a_kind(hand: &[&str]) -> usize {
    let v: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    for i in 0..3 {
        let count = v.iter().filter(|&x| *x == v[i]).count();
        if count == 3 {
            return v[i];
        }
    }
    0
}

// 是否有四条的存在，如果有，返回其牌张的大小
// 如果不存在，返回0
fn four_of_a_kind(hand: &[&str]) -> usize {
    let v: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    /*let v = [
        card_value(hand[0]),
        card_value(hand[1]),
        card_value(hand[2]),
        card_value(hand[3]),
        card_value(hand[4]),
    ];*/
    let count = v.iter().filter(|&x| *x == v[0]).count();
    if count == 4 {
        return v[0];
    }
    let count = v.iter().filter(|&x| *x == v[1]).count();
    if count == 4 {
        return v[1];
    }
    0
}

fn is_royal_flush(hand: &[&str]) -> bool {
    is_straight(hand) && is_flush(hand) && card_value(hand[4]) == 14 //最大一张是A
}

fn is_straight_flush(hand: &[&str]) -> bool {
    is_straight(hand) && is_flush(hand)
}

// 是顺子？
fn is_straight(hand: &[&str]) -> bool {
    let mut v: Vec<usize> = hand.iter().map(|x| card_value(x)).collect();
    v.sort();
    let diff1 = v[1] - v[0];
    let diff2 = v[2] - v[1];
    let diff3 = v[3] - v[2];
    let diff4 = v[4] - v[3];
    diff1 == 1 && diff1 == diff2 && diff2 == diff3 && diff3 == diff4
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

fn highest_card(hand: &[&str]) -> usize {
    let mut highest_value = 0;
    for card in hand {
        let value = card_value(card);
        //print!("{:?} ", card);
        if value > highest_value {
            highest_value = value;
        }
    }
    highest_value
}

/*
fn compare_cards(card1 :&str, card2: &str) -> i32 {
    let value1 = card_value(card1) ;
    let value2 = card_value(card2) ;
    if value1 > value2
}
*/

fn card_value(card: &str) -> usize {
    let ch = card.chars().next().unwrap();
    let all_cards = "..23456789TJQKA";
    all_cards.chars().position(|c| c == ch).unwrap()
}
