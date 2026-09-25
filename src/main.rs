mod enums;
mod components;
mod events;
mod resources;
mod states;

use bevy::ecs::schedule::LogLevel;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use crate::enums::suit::*;
use crate::components::card::*;
use crate::components::score_text::*;
use crate::events::match_event::*;
use crate::events::play_card::*;
use crate::resources::score::*;
use crate::states::game_phase::*;

fn main() {
    App::new()
        .edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .init_state::<GamePhase>()
        .init_resource::<Score>()
        .register_type::<Card>()
        .register_type::<Score>()
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_score_text)
        .add_systems(OnEnter(GamePhase::Playing), (reset_score, spawn_cards))
        .add_systems(OnEnter(GamePhase::GameOver), spawn_game_over_text)
        .add_systems(Update, (click_cards, check_game_over).run_if(in_state(GamePhase::Playing)))
        .add_systems(Update, restart.run_if(in_state(GamePhase::GameOver)).ambiguous_with(check_game_over))
        .add_systems(Update, update_score_text.run_if(resource_changed::<Score>))
        .add_observer(play_observe)
        .add_observer(score_observe)
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
                rank: 1,
                selected: false
            },
            Sprite {
                image: asset_server.load("images/card.png"),
                custom_size: Some(Vec2::new(100.0, 150.0)),
                ..default()
            },
            Transform::from_xyz(-180.0 + (item as f32 * 120.0), 0.0, 0.0),
            InHand,
            DespawnOnExit(GamePhase::GameOver),
        ));
    }
}

fn click_cards (
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    cards_query: Query<(Entity, &Transform), With<InHand>>,
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
            commands.trigger(PlayCard { card: entity });
        }
    }
}

fn play_observe (
    event: On<PlayCard>,
    mut commands: Commands,
    cards_query: Query<&Card>,
    table_query: Query<(Entity, &Card), With<OnTable>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let Ok(played) = cards_query.get(event.card) else { return; };

    if let Some((table_card, _)) = table_query.iter().find(|(_, c)| c.rank == played.rank) {
        commands.trigger(MatchEvent {
            hand_card: event.card,
            table_card: table_card,
        });
    } else {
        commands.entity(event.card).remove::<InHand>().insert(OnTable);

        let Ok(mut sprite) = sprite_query.get_mut(event.card) else { return; };
        sprite.color = bevy::color::palettes::css::GRAY.into();
    }
}

fn score_observe (
    event: On<MatchEvent>,
    mut commands: Commands,
    mut score: ResMut<Score>
) {
    score.value += 1;
    println!("得牌！目前分數：{}", score.value);
    commands.entity(event.hand_card).despawn();
    commands.entity(event.table_card).despawn();
}

fn spawn_score_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(12),
            ..default()
        },
        ScoreText,
    ));
}

fn update_score_text(
    score: Res<Score>,
    mut text_query: Query<&mut Text, With<ScoreText>>
) {
    let Ok(mut text) = text_query.single_mut() else { return; };
    **text = format!("Score: {}", score.value);
}

fn check_game_over(
    hand_query: Query<(), With<InHand>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if hand_query.is_empty() {
        next_phase.set(GamePhase::GameOver);
    }
}

fn reset_score(mut score:ResMut<Score>) {
    score.value = 0;
}

fn spawn_game_over_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Game Over - press R to restart"),
        TextFont {
            font_size: FontSize::Px(32.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: px(60),
            right: px(12),
            ..default()
        },
        DespawnOnExit(GamePhase::GameOver),
    ));
}

fn restart (
    keys: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        next_phase.set(GamePhase::Playing);
    }
}
