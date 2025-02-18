use crate::constants::figure::*;
use bevy::prelude::*;

pub mod animation;
pub mod big_t_shape;
pub mod cube;
pub mod line;
pub mod spawner;
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
}
