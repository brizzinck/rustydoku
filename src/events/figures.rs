use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct StartDragging(pub Entity);

#[derive(Debug, Event)]
pub struct EndDragging(pub Entity);

#[derive(Debug, Event)]
pub struct Placing(pub Entity);
