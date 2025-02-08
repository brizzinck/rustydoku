use super::{big_t_shape, cube, square, start_dragging, t_shape, Figure, FigureBounds, FigureType};
use bevy::prelude::*;
use rand::Rng;

pub(crate) fn random_spawn_figure(commands: &mut Commands, position: Vec2) -> Entity {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => cube::spawn(commands, position),
        1 => square::spawn(commands, position),
        2 => t_shape::spawn(commands, position),
        3 => big_t_shape::spawn(commands, position),
        _ => cube::spawn(commands, position),
    }
}

pub(super) fn spawn_empty_figure(
    commands: &mut Commands,
    position: Vec2,
    squares_position: &[Vec2],
) -> Entity {
    let mut rng = rand::thread_rng();

    let rotation_angle = match rng.gen_range(0..3) {
        0 => 90.0_f32.to_radians(),
        1 => 180.0_f32.to_radians(),
        2 => 270.0_f32.to_radians(),
        _ => 0.0,
    };

    let rotation = Quat::from_rotation_z(rotation_angle);

    let first_rotated = (rotation * squares_position[0].extend(0.)).truncate();
    let mut bounds_min = first_rotated;
    let mut bounds_max = first_rotated;

    for &offset in squares_position.iter() {
        let rotated = (rotation * offset.extend(0.)).truncate();
        bounds_min = bounds_min.min(rotated);
        bounds_max = bounds_max.max(rotated);
    }

    commands
        .spawn((
            FigureType::TShape,
            Figure::default(),
            Transform {
                translation: Vec3::new(position.x, position.y, 1.),
                rotation,
                ..Default::default()
            },
            FigureBounds {
                min: bounds_min,
                max: bounds_max,
            },
            PickingBehavior::default(),
            InheritedVisibility::default(),
        ))
        .observe(start_dragging)
        .id()
}
