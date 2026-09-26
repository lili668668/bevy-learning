use bevy::prelude::*;
use game_core::event::GameEvent;
use crate::components::*;
use crate::events::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score_text)
            .add_observer(render_hud);
    }
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

fn render_hud(
    event: On<CoreEvent>,
    mut commands: Commands,
    mut score_query: Query<&mut Text, With<ScoreText>>,
    game_over_query: Query<Entity, With<GameOverText>>,
) {
    match &event.0 {
        GameEvent::ScoreChanged(score) => {
            let Ok(mut text) = score_query.single_mut() else { return; };
            **text = format!("Score: {score}");
        }
        GameEvent::GameOver => {
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
                GameOverText,
            ));
        }
        GameEvent::Dealt(_) => {
            for entity in game_over_query.iter() {
                commands.entity(entity).despawn();
            }
        }
        _ => {}
    }
}
