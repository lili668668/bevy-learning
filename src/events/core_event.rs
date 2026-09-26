use bevy::prelude::*;
use game_core::event::GameEvent;

#[derive(Event)]
pub struct CoreEvent(pub GameEvent);
