use crate::{
    components::figure::{square::SquareComponent, FigureBoundsComponent, FigureComponent},
    constants::figure::*,
    resource::figure_spawner::FigureSpawnerResource,
    states::figure::FigureAnimationState,
};
use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng, Rng};

impl FigureComponent {
    /// Spawns a random figure at the specified absolute position.
    ///
    /// This function selects a figure at random from the available figures defined in the `FIGURES` constant,
    /// using a weighted probability distribution based on each figure's `weight` field.
    ///
    /// The steps performed are:
    /// 1. **Weight Calculation:**  
    ///    It gathers all figure weights from `FIGURES` and constructs a [`WeightedIndex`] distribution.
    ///    If the distribution cannot be created, it falls back to a simple random index.
    ///
    /// 2. **Figure Selection:**  
    ///    The selected figure's shape and name are then used to spawn the actual figure by calling
    ///    [`spawn_figure`].
    ///
    /// # Parameters
    ///
    /// - `commands`: A mutable reference to Bevy's `Commands`.
    /// - `absolute_position`: A [`Vec2`] representing the absolute position where the figure will be spawned.
    /// - `resource`: A reference to the [`FigureSpawnerResource`] that provides assets data.
    /// - `placeholder`: An [`Entity`] parent for the spawned figure.
    ///
    /// # Returns
    ///
    /// Returns an [`Entity`] corresponding to the spawned figure.
    pub(crate) fn random_spawn(
        commands: &mut Commands,
        absolute_position: Vec2,
        resource: &FigureSpawnerResource,
        placeholder: Entity,
    ) -> Entity {
        let mut rng = thread_rng();

        let weights: Vec<u32> = FIGURES.iter().map(|figure| figure.weight).collect();

        // Attempt to create a weighted index; fall back to a simple random index on error.
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

    /// Spawns an empty figure container at a given position with a random rotation for its squares.
    ///
    /// This function is responsible for creating a figure entity without its individual square children.
    /// It randomly selects one of three rotation angles (0°, 90°, 180°, or 270°) and applies the rotation to figure.
    /// It then calculates the bounds of the figure by rotating the first square position,
    /// and iteratively adjusting the minimum and maximum bounds based on the rotated positions.
    ///
    /// The spawned figure entity is also set up to observe the `start_dragging` event.
    ///
    /// # Parameters
    ///
    /// - `commands`: A mutable reference to Bevy's `Commands`.
    /// - `position`: A [`Vec2`] indicating where the figure container should be spawned.
    /// - `squares_position`: A slice of [`Vec2`] positions representing each square's offset relative to the figure's origin
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - The [`Entity`] of the spawned figure container.
    /// - The randomly selected [`Quat`] rotation applied to the figure
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

        let first_rotated = (rotation * squares_position[0].extend(1.)).truncate();
        let mut bounds_min = first_rotated;
        let mut bounds_max = first_rotated;

        for &offset in squares_position.iter().skip(1) {
            let rotated = (rotation * offset.extend(1.)).truncate();
            bounds_min = bounds_min.min(rotated);
            bounds_max = bounds_max.max(rotated);
        }

        (
            commands
                .spawn(FigureComponent::create(
                    position,
                    rotation,
                    FigureBoundsComponent::new(bounds_min, bounds_max),
                ))
                .observe(FigureComponent::start_dragging)
                .id(),
            rotation,
        )
    }

    /// Spawns a fully constructed figure at the specified absolute position.
    ///
    /// This function builds upon [`spawn_empty`] by first creating an empty figure container and then spawning
    /// each individual square as a child of the container.  
    ///
    /// The steps involved are:
    /// 1. **Empty Figure Creation:**  
    ///    Calls [`spawn_empty`] to create the figure container and obtain its rotation.
    ///
    /// 2. **Figure Initialization:**  
    ///    Instantiates a new [`FigureComponent`] with an empty vector for square entities,
    ///    the provided square positions, an initial animation state of `SpawnUpScaling`, and the provided placeholder.
    ///
    /// 3. **Square Spawning:**  
    ///    Iterates over each offset in `squares_position`, spawning a square as a child of the figure container
    ///    using [`SquareComponent::spawn_as_child`], and collects the resulting child entities.
    ///
    /// 4. **Component Insertion:**  
    ///    Inserts the completed [`FigureComponent`] and a [`Name`] component (using the given `name`)
    ///    into the figure container entity (name used for debugging in the Bevy Inspector).
    ///
    /// 5. **Logging & Return:**  
    ///    Logs the successful spawn with details and returns the entity of the spawned figure.
    ///
    /// # Parameters
    ///
    /// - `commands`: A mutable reference to Bevy's `Commands`.
    /// - `absolute_position`: A [`Vec2`] indicating where the figure should be spawned.
    /// - `squares_position`: A slice of [`Vec2`] positions representing each square's offset relative to the figure's origin.
    /// - `name`: A static string slice representing the name of the figure.
    /// - `resource`: A reference to the [`FigureSpawnerResource`] for additional configuration during square spawning.
    /// - `placeholder`: An [`Entity`] used as a placeholder during the figure's construction.
    ///
    /// # Returns
    ///
    /// Returns the [`Entity`] of the spawned figure.
    pub(crate) fn spawn_figure(
        commands: &mut Commands,
        absolute_position: Vec2,
        squares_position: &[Vec2],
        name: &'static str,
        resource: &FigureSpawnerResource,
        placeholder: Entity,
    ) -> Entity {
        let (parent, rotation) =
            FigureComponent::spawn_empty(commands, absolute_position, squares_position);

        let mut figure = FigureComponent {
            squares_entity: Vec::with_capacity(squares_position.len()),
            squares_position: squares_position.to_vec(),
            state_animation: FigureAnimationState::SpawnUpScaling,
            placeholder,
        };

        for &offset in squares_position.iter() {
            let child =
                SquareComponent::spawn_as_child(commands, parent, offset, rotation, resource);
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
