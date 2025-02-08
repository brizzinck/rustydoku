use bevy::prelude::*;

use crate::components::{figures::spawner::random_spawn_figure, map::TILE_SIZE};

const SIZE: usize = 3;

const FIGURE_POSITIONS: [(f32, f32); SIZE] = [
    (TILE_SIZE * 3.45, TILE_SIZE * -7.),
    (0., TILE_SIZE * -7.),
    (TILE_SIZE * -3.45, TILE_SIZE * -7.),
];

#[derive(Resource, Default)]
pub struct FigureSpawner {
    pub figures: u32,
}

impl FigureSpawner {
    pub fn remove(&mut self) {
        self.figures -= 1;
    }
}

pub fn spawn_figures(mut commands: Commands, mut figure_spawner: ResMut<FigureSpawner>) {
    if figure_spawner.figures == 0 {
        for &position in FIGURE_POSITIONS.iter() {
            let _ = random_spawn_figure(&mut commands, Vec2::new(position.0, position.1));

            figure_spawner.figures += 1;
        }
    }
}
