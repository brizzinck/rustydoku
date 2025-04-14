use crate::{
    components::{
        figure::{square::SquareComponent, FigureComponent},
        world::map::TileComponent,
    },
    constants::square::*,
    events::figure::FigureDeniedPlacingEvent,
    states::gameplay::GameState,
};
use bevy::prelude::*;

impl FigureComponent {
    /// Attempts to place a figure on the game board.
    ///
    /// This function is invoked when the game state is `GameState::Placing(figure)`.
    /// It validates whether the figure's individual squares can be placed on corresponding
    /// valid tiles in the map. The process follows these steps:
    ///
    /// 1. **Figure Retrieval:**  
    ///    Retrieves the current figure from the `figure_query` resource.
    ///
    /// 2. **Tile Collection:**  
    ///    Iterates over all available tiles (from `tile_query`) and collects their components,
    ///    transforms, and entity identifiers into a vector for later use.
    ///
    /// 3. **Square Placement Validation:**  
    ///    For each square entity in the figure (listed in `figure.squares_entity`), it:
    ///    - Obtains the square's transform from `square_query`.
    ///    - Checks if the square can be placed on a tile using [`SquareComponent::check_for_place`],
    ///      comparing the square's transform with the available tile data.
    ///    - Collects the tile entity for each square that can be placed.
    ///
    /// 4. **Validation Check:**  
    ///    If the number of valid placement tiles does not match the number of squares, or if the
    ///    figure's animation state is not default (via `figure.state_animation.is_default()`), the
    ///    placement is considered invalid:
    ///    - The game state is updated to `GameState::Idle`.
    ///    - A [`FigureDeniedPlacingEvent`] is emitted.
    ///    - The function exits early.
    ///
    /// 5. **Successful Placement:**  
    ///    If all squares are valid for placement:
    ///    - Each square is detached from its current parent.
    ///    - For each square, a corresponding tile is popped from the valid tiles list.
    ///    - The tile's [`TileComponent`] is updated to reference the square.
    ///    - The square is re-parented to the tile entity.
    ///    - The square's local transform is set to `SQUARE_PLACED_POSITION` to position it correctly.
    ///
    /// 6. **State Transition:**  
    ///    Finally, the game state is updated to `GameState::Placed` with the placed figure.
    ///
    /// # Parameters
    ///
    /// - `commands`: A mutable Bevy's `Commands`.
    /// - `current_state`: `A resource representing the current game state; the function proceeds only if
    ///    the state is `GameState::Placing(figure)`.`
    /// - `next_state`: A mutable resource used to set the next game state based on the outcome.
    /// - `square_query`: A query to retrieve each square's entity, its global transform, and a mutable local transform.
    /// - `tile_query`: A query to access each tile's [`TileComponent`], global transform, entity, and mutable [`Sprite`].
    /// - `figure_query`: A query to retrieve the [`FigureComponent`] for the figure being placed.
    /// - `event_denied`: An event writer used to emit a [`FigureDeniedPlacingEvent`] if placement validation fails.
    ///
    /// # Events
    ///
    /// - **Figure Denied:** If placement validation fails (i.e., not all squares have a valid tile
    ///   or the figure's state animation is non-default), a [`FigureDeniedPlacingEvent`] is sent.
    ///
    /// # State Transitions
    ///
    /// - **On Failure:** The game state transitions to `GameState::Idle`.
    /// - **On Success:** The game state transitions to `GameState::Placed` with the successfully placed figure.
    pub(crate) fn placing(
        mut commands: Commands,
        current_state: Res<State<GameState>>,
        mut next_state: ResMut<NextState<GameState>>,
        mut square_query: Query<(Entity, &GlobalTransform, &mut Transform)>,
        mut tile_query: Query<(&mut TileComponent, &GlobalTransform, Entity, &mut Sprite)>,
        figure_query: Query<&mut FigureComponent>,
        mut event_denied: EventWriter<FigureDeniedPlacingEvent>,
    ) {
        if let GameState::Placing(figure) = current_state.get() {
            let placed = figure;
            if let Ok(figure) = figure_query.get(*figure) {
                let all_tiles = tile_query
                    .iter()
                    .map(|(tile, global, entity, _)| (tile, global, entity))
                    .collect::<Vec<_>>();

                let mut tiles = vec![];

                for &square_entity in figure.squares_entity.iter() {
                    if let Ok((_, transform, _)) = square_query.get_mut(square_entity) {
                        if let Some(entity) =
                            SquareComponent::check_for_place(transform, &all_tiles)
                        {
                            tiles.push(entity);
                        }
                    }
                }

                if tiles.len() != figure.squares_entity.len()
                    || !figure.state_animation.is_default()
                {
                    next_state.set(GameState::Idle);
                    event_denied.send(FigureDeniedPlacingEvent(*placed));
                    trace!("Figure denied placing.");
                    return;
                }

                for &square in &figure.squares_entity {
                    if let Ok((square_entity, _, mut square_local_transform)) =
                        square_query.get_mut(square)
                    {
                        commands.entity(square_entity).remove_parent();

                        if let Some(tile_entity) = tiles.pop() {
                            if let Ok((mut tile, _, _, mut sprite)) =
                                tile_query.get_mut(tile_entity)
                            {
                                tile.square = Some(square_entity);
                                sprite.image = tile.default_image.clone();

                                commands.entity(square_entity).set_parent(tile_entity);

                                *square_local_transform =
                                    Transform::from_translation(SQUARE_PLACED_POSITION);
                            }
                        }
                    }
                }

                next_state.set(GameState::Placed(*placed));

                trace!("Figure placed successfully.");
            }
        }
    }
}
