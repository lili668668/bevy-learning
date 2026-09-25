use bevy::prelude::*;
use crate::enums::suit::*;
use crate::components::card::*;
use crate::events::match_event::*;
use crate::events::play_card::*;
use crate::states::game_phase::*;

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Card>()
            .add_systems(OnEnter(GamePhase::Playing), spawn_cards)
            .add_systems(Update, click_cards.run_if(in_state(GamePhase::Playing)))
            .add_observer(play_observe);
    }
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
