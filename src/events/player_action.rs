use bevy::prelude::*;
use game_core::action::Action;

#[derive(Event)]
pub struct PlayerAction(pub Action);
