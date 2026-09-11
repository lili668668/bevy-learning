use bevy::prelude::*;

#[derive(Event)]
pub struct MatchEvent {
    pub hand_card: Entity,
    pub table_card: Entity,
}
