use super::{
    square::{self},
    start_dragging, Figure, FigureType,
};
use bevy::prelude::*;

pub fn spawn(mut commands: Commands) {
    let parent = commands
        .spawn((
            FigureType::Cube,
            Figure::default(),
            Transform::from_xyz(0.0, -300.0, 1.0),
            PickingBehavior::default(),
            InheritedVisibility::default(),
        ))
        .observe(start_dragging)
        .id();

    for x in -1..1 {
        for y in -1..1 {
            square::spawn(
                &mut commands,
                parent,
                Vec2::new(x as f32 + 0.5, y as f32 + 0.5),
            );
        }
    }
}
