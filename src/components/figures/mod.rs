use crate::states::figure::{placeholder::StatePlaceholderAnimation, StateFigureAnimation};
use bevy::prelude::*;
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub mod big_t_shape;
pub mod cube;
pub mod line;
pub mod square;
pub mod t_shape;

#[derive(Component, Debug, Copy, Clone)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct FigureBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Clone, Component, Default, Debug)]
pub struct Figure {
    pub squares_entity: Vec<Entity>,
    pub squares_position: Vec<Vec2>,
    pub state_animation: StateFigureAnimation,
}

#[derive(Component)]
pub struct FigureZone {
    pub placeholders: Vec<Entity>,
    pub placeholder_state_animation: StatePlaceholderAnimation,
}

#[derive(Component)]
pub struct Placeholder;
