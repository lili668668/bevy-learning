use bevy::prelude::*;
use crate::enums::suit::*;

#[derive(Component)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}
