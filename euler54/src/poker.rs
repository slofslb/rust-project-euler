#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Spade,   // 黑桃
    Heart,   // 红桃
    Diamond, // 方块
    Club,    // 梅花
}

impl Display for Suit {
    // 只用一个字母表示: S,H,D,C
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let name = format!("{:?}", self);
        // 注意下面这一句后面千万别习惯性地加个分号，否则出现的编译错误让人好困惑！
        write!(f, "{}", &name[..1])
    }
}

#[derive(Debug)]
pub struct Card {
    value: u8, // 用2到14表示2, 3, ..., 10, J, Q, K, A
    suit: Suit,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let first_char = "..23456789TJQKA".chars().nth(self.value as usize).unwrap();
        write!(f, "{}{}", first_char, self.suit)
    }
}

impl Card {
    pub fn new(str_card: &str) -> Card {
        let first_char = str_card.chars().next().unwrap();
        let card_value = "..23456789TJQKA"
            .chars()
            .position(|c| c == first_char)
            .unwrap() as u8;
        let second_char = str_card.chars().nth(1).unwrap();
        let card_suit = if second_char == 'S' {
            Suit::Spade
        } else if second_char == 'H' {
            Suit::Heart
        } else if second_char == 'D' {
            Suit::Diamond
        } else {
            Suit::Club
        };
        Card {
            value: card_value,
            suit: card_suit,
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

use itertools::Itertools;
impl Hand {
    pub fn new(str_cards: &str) -> Hand {
        let mut v = vec![];
        for s in str_cards.split(' ') {
            v.push(Card::new(s));
        }
        Hand { cards: v }
    }

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

use std::fmt::{Display, Error, Formatter};
impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let str_cards = self.cards.iter().map(|x| x.to_string()).join(" ");
        write!(f, "{}", str_cards)
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

#[derive(Debug)]
pub enum HandType {
    HighCard(u8, u8, u8, u8, u8), // 单牌
    //对子
    OnePair {
        value_pair: u8,
        max_remain: u8, // 除了对子之外，剩下最大的牌点
    },
    //两对
    TwoPairs {
        high_pair: u8,  // 最大的一对
        low_pair: u8,   // 最小的一对
        max_remain: u8, // 除了两对之外，剩下最大的牌点
    },
    KindThree(u8), // 三条
    Straight(u8),  // 顺子
    Flush(u8),     // 同花
    //葫芦，即三条带一个对子
    FullHouse {
        value_kind_three: u8,
        value_pair: u8,
    },
    KindFour(u8),      // 四条
    StraightFlush(u8), // 同花顺
    RoyalFlush,        // 同花大顺
}

use HandType::*;

impl HandType {
    pub fn ranking1(&self) -> u8 {
        match self {
            HighCard(_, _, _, _, _) => 0,
            OnePair {
                value_pair: _,
                max_remain: _,
            } => 1,
            TwoPairs {
                high_pair: _,
                low_pair: _,
                max_remain: _,
            } => 2,
            KindThree(_) => 3,
            Straight(_) => 4,
            Flush(_) => 5,
            FullHouse {
                value_kind_three: _,
                value_pair: _,
            } => 6,
            KindFour(_) => 7,
            StraightFlush(_) => 8,
            RoyalFlush => 9,
        }
    }

    pub fn ranking2(&self) -> u64 {
        match self {
            HighCard(a, b, c, d, e) => {
                (*a as u64) * 100_000_000
                    + (*b as u64) * 1_000_000
                    + (*c as u64) * 10_000
                    + (*d as u64) * 100
                    + (*e as u64)
            }
            OnePair {
                value_pair: a,
                max_remain: b,
            } => (*a as u64) * 100 + (*b as u64),
            TwoPairs {
                high_pair: a,
                low_pair: b,
                max_remain: c,
            } => (*a as u64) * 10000 + (*b as u64) * 100 + (*c as u64),
            KindThree(a) => *a as u64,
            Straight(a) => *a as u64,
            Flush(a) => *a as u64,
            FullHouse {
                value_kind_three: a,
                value_pair: b,
            } => (*a as u64) * 100 + (*b as u64),
            KindFour(a) => *a as u64,
            StraightFlush(a) => *a as u64,
            RoyalFlush => 0,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ranking1()
            .cmp(&other.ranking1())
            .then(self.ranking2().cmp(&other.ranking2()))
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for HandType {}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.ranking1() == other.ranking1() && self.ranking2() == other.ranking2()
    }
}
