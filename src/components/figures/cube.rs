use super::{start_dragging, Figure, FigureType, FIGURE_SIZE};
use bevy::prelude::*;

pub fn spawn(mut commands: Commands) {
    let parent = commands
        .spawn((
            FigureType::Cube,
            Figure::default(),
            Transform::from_xyz(0.0, 0.0, 1.0),
            PickingBehavior::default(),
            InheritedVisibility::default(),
        ))
        .observe(start_dragging)
        .id();

    for x in -1..=1 {
        for y in -1..=1 {
            commands
                .spawn((
                    Sprite {
                        custom_size: Some(Vec2::new(FIGURE_SIZE, FIGURE_SIZE)),
                        ..Default::default()
                    },
                    Transform::from_xyz(x as f32 * FIGURE_SIZE, y as f32 * FIGURE_SIZE, 1.0),
                ))
                .set_parent(parent);
        }
    }
}
