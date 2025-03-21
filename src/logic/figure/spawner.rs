use crate::{
    components::figure::{square::Square, Figure, FigureBounds},
    constants::figure::*,
    resource::figure_spawner::FigureSpawner,
    states::figure::StateFigureAnimation,
};
use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng, Rng};

impl Figure {
    pub(crate) fn random_spawn(
        commands: &mut Commands,
        absolute_position: Vec2,
        resource: &FigureSpawner,
        placeholder: Entity,
    ) -> Entity {
        let mut rng = thread_rng();

        let weights: Vec<u32> = FIGURES.iter().map(|figure| figure.weight).collect();

        let figure_index = match WeightedIndex::new(&weights) {
            Ok(dist) => dist.sample(&mut rng),
            Err(_) => {
                warn!("WeightedIndex failed! Using random instead");
                rand::random::<usize>() % FIGURES.len()
            }
        };

        let selected_figure = &FIGURES[figure_index];

        Self::spawn_figure(
            commands,
            absolute_position,
            selected_figure.shape,
            selected_figure.name,
            resource,
            placeholder,
        )
    }

    pub(crate) fn spawn_empty(
        commands: &mut Commands,
        position: Vec2,
        squares_position: &[Vec2],
    ) -> (Entity, Quat) {
        let mut rng = rand::thread_rng();

        let rotation_angle = match rng.gen_range(0..3) {
            0 => 90.0_f32.to_radians(),
            1 => 180.0_f32.to_radians(),
            2 => 270.0_f32.to_radians(),
            _ => 0.,
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
                .spawn(Figure::create(
                    position,
                    rotation,
                    FigureBounds::new(bounds_min, bounds_min),
                ))
                .observe(Figure::start_dragging)
                .id(),
            rotation,
        )
    }

    pub(crate) fn spawn_figure(
        commands: &mut Commands,
        absolute_position: Vec2,
        squares_position: &[Vec2],
        name: &'static str,
        resource: &FigureSpawner,
        placeholder: Entity,
    ) -> Entity {
        let (parent, rotation) = Figure::spawn_empty(commands, absolute_position, squares_position);

        let mut figure = Figure {
            squares_entity: Vec::with_capacity(squares_position.len()),
            squares_position: squares_position.to_vec(),
            state_animation: StateFigureAnimation::SpawnUpScaling,
            placeholder,
        };

        for &offset in squares_position.iter() {
            let child = Square::spawn_as_child(commands, parent, offset, rotation, resource);
            figure.squares_entity.push(child);
        }

        commands.entity(parent).insert(figure);
        commands.entity(parent).insert(Name::new(name));

        trace!(
            "Figure {} spawned, position: {:?}, rotation: {:?}",
            name,
            absolute_position,
            rotation
        );

        parent
    }
}
