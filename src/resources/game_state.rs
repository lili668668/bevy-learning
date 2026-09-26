use bevy::prelude::*;
use game_core::game::Game;

#[derive(Resource)]
pub struct GameState(pub Game);
