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
        .add_systems(Update, move_cards)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_cards(mut commands: Commands) {
    let suits = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

    for (item, suit) in suits.into_iter().enumerate() {
        commands.spawn((
            Card {
                suit: suit,
                rank: 1
            },
            Sprite {
                color: bevy::color::palettes::css::CRIMSON.into(),
                custom_size: Some(Vec2::new(100.0, 150.0)),
                ..default()
            },
            Transform::from_xyz(-180.0 + (item as f32 * 120.0), 0.0, 0.0)
        ));
    }
}

fn move_cards (
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Card>>,
) {
    for mut transform in &mut query {
        transform.translation.y += 50.0 * time.delta_secs();
    }
}
