use bevy::prelude::*;
use crate::enums::suit::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
    pub selected: bool,
}
