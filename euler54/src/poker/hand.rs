use super::card::*;
use super::hand_type::*;

use std::fmt::{Display, Error, Formatter};
pub struct Hand {
    pub cards: Vec<Card>,
}

use itertools::Itertools;
impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let str_cards = self.cards.iter().map(|x| x.to_string()).join(" ");
        write!(f, "{}", str_cards)
    }
}

impl Hand {
    pub fn new(str_cards: &str) -> Hand {
        let mut v = vec![];
        for s in str_cards.split(' ') {
            v.push(Card::new(s));
        }
        Hand { cards: v }
    }
}

use std::cmp::{Ord, Ordering};
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type().cmp(&other.hand_type())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// 这一条实现Eq的空语句折腾了好长时间
// impl Ord for Hand {
//      ^^^ the trait `std::cmp::Eq` is not implemented for `poker::Hand`
impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type().eq(&other.hand_type())
    }
}

impl Hand {
    // 统计各种牌点出现的次数，例如：[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0]
    // 表示五张牌里有3张10，1张J(11点)，1张Q(12点)
    fn cards_stat(&self) -> Vec<u8> {
        // 对不同的牌计数
        let mut count_cards = vec![0_u8; 15]; // 最大A，值是14
        let values: Vec<u8> = self.cards.iter().map(|x| x.value).collect();
        for v in values {
            count_cards[v as usize] += 1;
        }
        count_cards
    }

    // 是否同花？
    fn is_flush(&self) -> bool {
        self.cards.iter().map(|card| card.suit).all_equal()
    }

    // 是否顺子？
    fn is_straight(&self) -> bool {
        let mut v: Vec<u8> = self.cards.iter().map(|x| x.value).collect();
        v.sort();
        (0..4).all(|i| v[i + 1] - v[i] == 1) //两两之差都为1
    }

    pub fn hand_type(&self) -> HandType {
        // 最大的牌点
        let max_value = self.cards.iter().map(|x| x.value).max().unwrap();

        if self.is_flush() && self.is_straight() {
            if max_value == 14 {
                return HandType::RoyalFlush;
            } else {
                return HandType::StraightFlush(max_value);
            }
        } else if self.is_flush() {
            return HandType::Flush(max_value);
        } else if self.is_straight() {
            return HandType::Straight(max_value);
        }

        let stat = self.cards_stat();

        if let Some(a) = stat.iter().position(|&x| x == 4) {
            return HandType::KindFour(a as u8);
        } else if let Some(a) = stat.iter().position(|&x| x == 3) {
            // 有三条出现
            if let Some(b) = stat.iter().position(|&x| x == 2) {
                //如果还有一对，就是葫芦
                return HandType::FullHouse {
                    value_kind_three: a as u8,
                    value_pair: b as u8,
                };
            }
            return HandType::KindThree(a as u8);
        } else if let Some(a) = stat.iter().rposition(|&x| x == 2) {
            // 寻找最大的对子
            let b = stat.iter().position(|&x| x == 2).unwrap();
            let max_hight_card = stat.iter().rposition(|&x| x == 1).unwrap() as u8; // 单牌的最大值
            if a != b {
                //如果还有一对
                return HandType::TwoPairs {
                    high_pair: a as u8,
                    low_pair: b as u8,
                    max_remain: max_hight_card,
                };
            }
            return HandType::OnePair {
                value_pair: a as u8,
                max_remain: max_hight_card,
            };
        }

        // 当HighCard时，仅保存最大的牌点不行
        // 例如 ：A9753和A9752，仅比较最大的牌是不行的，还得依次比较最大的牌点才能出来结果
        // 所以要将5张牌全记录下来
        let mut c: Vec<_> = self.cards.iter().map(|x| x.value).collect();
        c.sort();
        HandType::HighCard(c[4], c[3], c[2], c[1], c[0])
    }
}
