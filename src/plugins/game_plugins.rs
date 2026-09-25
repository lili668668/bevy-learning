use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use crate::plugins::game_phase_plugin::*;
use crate::plugins::card_plugin::*;
use crate::plugins::score_plugin::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GamePhasePlugin)
            .add(CardPlugin)
            .add(ScorePlugin)
    }
}
