use crate::{constants::figure::*, states::figure::FigureAnimationState};
use bevy::prelude::*;

pub mod square;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

#[derive(Component, Debug, Copy, Clone)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct FigureBoundsComponent {
    pub min: Vec2,
    pub max: Vec2,
}

impl FigureBoundsComponent {
    pub(crate) fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    pub(crate) fn from(bounds: FigureBoundsComponent) -> Self {
        Self {
            min: bounds.min,
            max: bounds.max,
        }
    }
}

#[derive(Clone, Component, Debug)]
pub struct FigureComponent {
    pub squares_entity: Vec<Entity>,
    pub squares_position: Vec<Vec2>,
    pub state_animation: FigureAnimationState,
    pub placeholder: Entity,
}

impl FigureComponent {
    pub(crate) fn create(
        position: Vec2,
        rotation: Quat,
        bounds: FigureBoundsComponent,
    ) -> impl Bundle {
        (
            Transform {
                translation: Vec3::new(position.x, position.y, FIGURE_Z_POSITION),
                rotation,
                scale: Vec3::ZERO,
            },
            FigureBoundsComponent::from(bounds),
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
