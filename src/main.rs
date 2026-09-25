mod enums;
mod components;
mod events;
mod resources;
mod states;
mod plugins;

use bevy::prelude::*;
use crate::plugins::debug_plugin::*;
use crate::plugins::game_phase_plugin::*;
use crate::plugins::card_plugin::*;
use crate::plugins::score_plugin::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugin)
        .add_plugins(GamePhasePlugin)
        .add_plugins(CardPlugin)
        .add_plugins(ScorePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

