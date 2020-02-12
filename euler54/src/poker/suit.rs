#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Suit {
    Spade,   // 黑桃
    Heart,   // 红桃
    Diamond, // 方块
    Club,    // 梅花
}

use std::fmt::{Display, Error, Formatter};
impl Display for Suit {
    // 只用一个字母表示: S,H,D,C
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let name = format!("{:?}", self);
        // 注意下面这一句后面千万别习惯性地加个分号，否则出现的编译错误让人好困惑！
        write!(f, "{}", &name[..1])
    }
}
