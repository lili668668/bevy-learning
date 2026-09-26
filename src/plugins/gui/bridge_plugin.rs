use bevy::prelude::*;
use game_core::game::Game;
use crate::resources::*;
use crate::events::*;

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_game)
            .add_observer(apply_player_action);
    }
}

fn start_game(mut commands: Commands) {
    let (game, events) = Game::new();
    commands.insert_resource(GameState(game));
    for event in events {
        commands.trigger(CoreEvent(event));
    }
}

fn apply_player_action(
    action: On<PlayerAction>,
    mut commands: Commands,
    mut game: ResMut<GameState>,
) {
    for event in game.0.apply(action.0) {
        commands.trigger(CoreEvent(event));
    }
}
