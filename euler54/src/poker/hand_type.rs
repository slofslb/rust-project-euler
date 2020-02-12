use std::cmp::{Ord, Ordering};

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
            OnePair { .. } => 1,
            TwoPairs { .. } => 2,
            KindThree(_) => 3,
            Straight(_) => 4,
            Flush(_) => 5,
            FullHouse { .. } => 6,
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
