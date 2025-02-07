use super::{big_t_shape, cube, square, start_dragging, t_shape, Figure, FigureType};
use bevy::prelude::*;
use rand::Rng;

pub(crate) fn spawn_figures(mut commands: Commands) {
    square::spawn(&mut commands, Vec2::new(-80.0, 160.));
    cube::spawn(&mut commands, Vec2::new(-140.0, 140.));
    t_shape::spawn(&mut commands, Vec2::new(0., 160.));
    big_t_shape::spawn(&mut commands, Vec2::new(120., 160.));
}

pub(super) fn spawn_empty_figure(commands: &mut Commands, position: Vec2) -> Entity {
    let mut rng = rand::thread_rng();

    let rotation_angle = match rng.gen_range(0..3) {
        0 => 90.0_f32.to_radians(), // Convert degrees to radians
        1 => 180.0_f32.to_radians(),
        2 => 270.0_f32.to_radians(),
        _ => 0.0,
    };

    commands
        .spawn((
            FigureType::TShape,
            Figure::default(),
            Transform {
                translation: Vec3::new(position.x, position.y, 1.0),
                rotation: Quat::from_rotation_z(rotation_angle), // Apply rotation
                ..Default::default()
            },
            PickingBehavior::default(),
            InheritedVisibility::default(),
        ))
        .observe(start_dragging)
        .id()
}
