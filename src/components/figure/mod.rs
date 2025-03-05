use crate::{constants::figure::*, states::figure::StateFigureAnimation};
use bevy::prelude::*;

pub mod square;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct FigureBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl FigureBounds {
    pub(crate) fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    pub(crate) fn from(bounds: FigureBounds) -> Self {
        Self {
            min: bounds.min,
            max: bounds.max,
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct Figure {
    pub squares_entity: Vec<Entity>,
    pub squares_position: Vec<Vec2>,
    pub state_animation: StateFigureAnimation,
    pub placeholder: Entity,
}

impl Figure {
    pub(crate) fn create(position: Vec2, rotation: Quat, bounds: FigureBounds) -> impl Bundle {
        (
            Transform {
                translation: Vec3::new(position.x, position.y, FIGURE_Z_POSITION),
                rotation,
                scale: Vec3::ZERO,
            },
            FigureBounds::from(bounds),
            PickingBehavior::default(),
            InheritedVisibility::default(),
            Sprite {
                custom_size: Some(Vec2::new(MAX_FIGURE_SIZE, MAX_FIGURE_SIZE)),
                color: INTERACTIVE_ZONE_COLOR.into(),
                ..default()
            },
        )
    }
}
