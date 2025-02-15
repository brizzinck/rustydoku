use super::{big_t_shape, cube, line, square, start_dragging, t_shape, Figure, FigureBounds};
use bevy::prelude::*;
use rand::Rng;

pub(crate) fn random_spawn_figure(
    commands: &mut Commands,
    position: Vec2,
    assets: &Res<AssetServer>,
) -> Entity {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => cube::spawn(commands, position, assets),
        1 => square::spawn(commands, position, assets),
        2 => t_shape::spawn(commands, position, assets),
        3 => big_t_shape::spawn(commands, position, assets),
        4 => line::spawn(commands, position, assets),
        _ => cube::spawn(commands, position, assets),
    }
}

pub fn spawn_empty_figure(
    commands: &mut Commands,
    position: Vec2,
    squares_position: &[Vec2],
) -> (Entity, Quat) {
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

    (
        commands
            .spawn((
                Transform {
                    translation: Vec3::new(position.x, position.y, 1.),
                    rotation,
                    scale: Vec3::ONE,
                },
                FigureBounds {
                    min: bounds_min,
                    max: bounds_max,
                },
                PickingBehavior::default(),
                InheritedVisibility::default(),
            ))
            .observe(start_dragging)
            .id(),
        rotation,
    )
}

pub(super) fn spawn_figure(
    commands: &mut Commands,
    position: Vec2,
    squares_position: &[Vec2],
    name: &'static str,
    assets: &Res<AssetServer>,
) -> Entity {
    let (parent, rotation) = spawn_empty_figure(commands, position, squares_position);
    let mut figure = Figure {
        squares: Vec::with_capacity(squares_position.len()),
    };

    for &offset in squares_position.iter() {
        let child = square::spawn_child(commands, parent, offset, rotation, assets);
        figure.squares.push(child);
    }

    commands.entity(parent).insert(figure);
    commands.entity(parent).insert(Name::new(name));

    parent
}
