use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum GamePhase {
    #[default]
    Playing,
    GameOver,
}
