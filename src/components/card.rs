use bevy::prelude::*;
use game_core::card::CardId;

#[derive(Component)]
pub struct CardView(pub CardId);

