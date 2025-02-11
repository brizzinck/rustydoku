use super::{
    spawner::spawn_empty_figure,
    square::{self},
    Figure, FigureType,
};
use bevy::prelude::*;

pub fn spawn(commands: &mut Commands, position: Vec2) -> Entity {
    let squares_position = [
        Vec2::new(-1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, -1.0),
    ];

    let parent = spawn_empty_figure(commands, position, &squares_position);

    let mut figure = Figure {
        squares: Vec::with_capacity(squares_position.len()),
    };

    for &offset in &squares_position {
        let child = square::spawn_child(commands, parent, offset);
        figure.squares.push(child);
    }

    commands.entity(parent).insert(figure);
    commands.entity(parent).insert(FigureType::TShape);

    parent
}
