use crate::{constants::figure::*, states::figure::FigureAnimationState};
use bevy::prelude::*;

pub mod square;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

/// Defines the bounding box of a figure with minimum and maximum local coordinates.
#[derive(Component, Debug, Copy, Clone)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct FigureBoundsComponent {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

impl FigureBoundsComponent {
    /// Creates a new bounds component from min and max vectors.
    pub(crate) fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Creates a new bounds component from another bounds instance.
    pub(crate) fn from(bounds: FigureBoundsComponent) -> Self {
        Self {
            min: bounds.min,
            max: bounds.max,
        }
    }
}

/// Component representing a figure entity composed of multiple square entities.
#[derive(Clone, Component, Debug)]
pub struct FigureComponent {
    /// The list of entities representing the individual squares.
    pub squares_entity: Vec<Entity>,

    /// The relative positions of the squares within the figure.
    pub squares_position: Vec<Vec2>,

    /// The current animation state of the figure.
    pub state_animation: FigureAnimationState,

    /// The placeholder entity that this figure originated from.
    pub placeholder: Entity,
}

impl FigureComponent {
    /// Creates a new figure bundle with position, rotation, bounds, and interactive zone.
    ///
    /// # Arguments
    /// - `position`: World position where the figure will be spawned.
    /// - `rotation`: Rotation applied to the entire figure.
    /// - `bounds`: The bounding box of the figure.
    ///
    /// # Returns
    /// A bundle containing transform, sprite, bounds, and default visibility behavior.
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
