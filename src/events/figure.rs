use bevy::prelude::*;

/// Event triggered when a player begins dragging a figure.
///
/// Contains the [`Entity`] of the figure being dragged.
#[derive(Event)]
pub struct FigureTriggerDraggingEvent(pub Entity);

/// Event triggered when a player stops dragging (drops) a figure.
///
/// Contains the [`Entity`] of the figure being dropped.
#[derive(Event)]
pub struct FigureTriggerUpEvent(pub Entity);

/// Event triggered when an attempt to place a figure fails due to invalid placement.
///
/// Contains the [`Entity`] of the figure whose placement was denied.
#[derive(Event)]
pub struct FigureDeniedPlacingEvent(pub Entity);

/// Event triggered when a new figure is spawned into the game.
///
/// Contains the [`Entity`] of the newly spawned figure.
#[derive(Event)]
pub struct FigureSpawnedEvent(pub Entity);

/// Event triggered when a figure can't be placed anywhere on the current game board.
///
/// Contains the [`Entity`] of the placeholder associated with the unplaceable figure.
#[derive(Event)]
pub struct FigureCantPlacedEvent(pub Entity);

/// Event triggered when a figure is confirmed to be placeable on the game board.
///
/// Contains the [`Entity`] of the placeholder associated with the placeable figure.
#[derive(Event)]
pub struct FigureCanPlacedEvent(pub Entity);
