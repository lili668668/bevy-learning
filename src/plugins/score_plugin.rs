use bevy::prelude::*;
use crate::components::score_text::*;
use crate::events::match_event::*;
use crate::resources::score::*;
use crate::states::game_phase::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .register_type::<Score>()
            .add_systems(Startup, spawn_score_text)
            .add_systems(OnEnter(GamePhase::Playing), reset_score)
            .add_systems(Update, update_score_text.run_if(resource_changed::<Score>))
            .add_observer(score_observe);
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

fn reset_score(mut score:ResMut<Score>) {
    score.value = 0;
}
