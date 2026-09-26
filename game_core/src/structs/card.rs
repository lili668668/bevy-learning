use crate::enums::suit::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CardId(pub u32);
