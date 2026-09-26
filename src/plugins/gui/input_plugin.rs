use bevy::prelude::*;
use game_core::action::Action;
use crate::events::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, press_r_to_restart);
    }
}

fn press_r_to_restart(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        commands.trigger(PlayerAction(Action::Restart));
    }
}
