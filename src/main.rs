mod enums;
mod components;

use bevy::prelude::*;
use crate::enums::suit::*;
use crate::components::card::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_cards)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_cards(mut commands: Commands) {
    commands.spawn((
        Card {
            suit: Suit::Heart,
            rank: 1
        },
        Sprite {
            color: bevy::color::palettes::css::CRIMSON.into(),
            custom_size: Some(Vec2::new(100.0, 150.0)),
            ..default()
        },
        Transform::default(),
    ));
}
