use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Resource, Default)]
pub enum StateFigure {
    #[default]
    None,
    Dragging(Entity),
    Placing(Entity),
}
