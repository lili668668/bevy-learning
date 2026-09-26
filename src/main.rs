mod components;
mod events;
mod resources;
mod plugins;

use bevy::prelude::*;
use crate::plugins::crash_report_plugin::*;
use crate::plugins::debug_plugin::*;
use crate::plugins::gui::gui_plugins::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(CrashReportPlugin)
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "dev")]
    app.add_plugins(DebugPlugin);

    app.add_plugins(GuiPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

