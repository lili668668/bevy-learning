use bevy::prelude::*;

#[derive(Event)]
pub struct PlayCard {
    pub card: Entity,
}
