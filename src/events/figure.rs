use bevy::prelude::*;

#[derive(Event)]
pub struct FigureTriggerEnter;

#[derive(Event)]
pub struct FigureTriggerDragging(pub Entity);

#[derive(Event)]
pub struct FigureTriggerUp(pub Entity);
