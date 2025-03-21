use bevy::prelude::*;

#[derive(Event)]
pub struct FigureTriggerDraggingEvent(pub Entity);

#[derive(Event)]
pub struct FigureTriggerUpEvent(pub Entity);

#[derive(Event)]
pub struct FigureDeniedPlacingEvent(pub Entity);

#[derive(Event)]
pub struct FigureSpawnedEvent(pub Entity);

#[derive(Event)]
pub struct FigureCantPlacedEvent(pub Entity);

#[derive(Event)]
pub struct FigureCanPlacedEvent(pub Entity);
