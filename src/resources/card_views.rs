use bevy::prelude::*;
use game_core::card::CardId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct CardViews(pub HashMap<CardId, Entity>);
