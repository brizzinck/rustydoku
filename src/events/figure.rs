use bevy::prelude::*;

#[derive(Event)]
pub struct FigureTriggerEnter;

#[derive(Event)]
pub struct FigureTriggerDragging(pub Entity);

#[derive(Event)]
pub struct FigureMoving(pub Entity);

#[derive(Event)]
pub struct FigureTriggerUp(pub Entity);

#[derive(Event)]
pub struct FigureDeniedPlacing(pub Entity);

#[derive(Event)]
pub struct FigureSpawned(pub Entity);

#[derive(Event)]
pub struct FigureCantPlaced(pub Entity);

#[derive(Event)]
pub struct FigureCanPlaced(pub Entity);
