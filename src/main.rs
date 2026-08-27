mod enums;
mod components;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use crate::enums::suit::*;
use crate::components::card::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Card>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_cards)
        .add_systems(Update, move_cards)
        .add_systems(Update, click_cards)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_cards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let suits = [Suit::Spade, Suit::Heart, Suit::Diamond, Suit::Club];

    for (item, suit) in suits.into_iter().enumerate() {
        commands.spawn((
            Card {
                suit: suit,
                rank: 1
            },
            Sprite {
                image: asset_server.load("images/card.png"),
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

fn click_cards (
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    cards_query: Query<(Entity, &Transform), With<Card>>,
) {
    if !buttons.just_pressed(MouseButton::Left) { return; }

    let Ok(window) = windows.single() else { return; };
    let Some(cursor_position) = window.cursor_position() else { return; };

    let Ok((camera, camera_transform)) = camera_query.single() else { return; };

    let Ok(world_postion) = camera.viewport_to_world_2d(camera_transform, cursor_position) else { return; };

    for (entity, transform) in cards_query.iter() {
        let card_size = Vec2::new(100.0, 150.0);
        let position = transform.translation.truncate();
        let half = card_size / 2.0;
        
        if world_postion.x >= position.x - half.x && world_postion.x <= position.x + half.x &&
           world_postion.y >= position.y - half.y && world_postion.y <= position.y + half.y
        {
            println!("觸牌： {:?}", entity);
        }
    }
}
