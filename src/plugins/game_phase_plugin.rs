use bevy::prelude::*;
use crate::components::card::*;
use crate::states::game_phase::*;

pub struct GamePhasePlugin;

impl Plugin for GamePhasePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GamePhase>()
            .add_systems(OnEnter(GamePhase::GameOver), spawn_game_over_text)
            .add_systems(Update, check_game_over.run_if(in_state(GamePhase::Playing)))
            .add_systems(Update, restart.run_if(in_state(GamePhase::GameOver)).ambiguous_with(check_game_over));
    }
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

fn check_game_over(
    hand_query: Query<(), With<InHand>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if hand_query.is_empty() {
        next_phase.set(GamePhase::GameOver);
    }
}

fn restart (
    keys: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        next_phase.set(GamePhase::Playing);
    }
}
