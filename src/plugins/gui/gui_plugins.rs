use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use crate::plugins::gui::bridge_plugin::*;
use crate::plugins::gui::card_view_plugin::*;
use crate::plugins::gui::hud_plugin::*;
use crate::plugins::gui::input_plugin::*;

pub struct GuiPlugins;

impl PluginGroup for GuiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BridgePlugin)
            .add(CardViewPlugin)
            .add(HudPlugin)
            .add(InputPlugin)
    }
}
