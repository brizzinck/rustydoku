use crate::{
    components::{
        figures::{square::Square, Figure},
        map::Tile,
    },
    constants::figure::*,
    events::figure::FigureDeniedPlacing,
    states::gameplay::StateGame,
};
use bevy::prelude::*;

impl Figure {
    pub(crate) fn placing(
        mut commands: Commands,
        current_state: Res<State<StateGame>>,
        mut next_state: ResMut<NextState<StateGame>>,
        mut square_query: Query<(Entity, &GlobalTransform, &mut Transform)>,
        mut tile_query: Query<(&mut Tile, &GlobalTransform, Entity, &mut Sprite)>,
        figure_query: Query<&mut Figure>,
        mut event_writer: EventWriter<FigureDeniedPlacing>,
    ) {
        if let StateGame::Placing(figure) = current_state.get() {
            let placed = figure;
            if let Ok(figure) = figure_query.get(*figure) {
                let all_tiles = tile_query
                    .iter()
                    .map(|(tile, global, entity, _)| (tile, global, entity))
                    .collect::<Vec<_>>();

                let mut tiles = vec![];

                for &square_entity in figure.squares_entity.iter() {
                    if let Ok((_, transform, _)) = square_query.get_mut(square_entity) {
                        if let Some(entity) = Square::check_for_place(transform, &all_tiles) {
                            tiles.push(entity);
                        }
                    }
                }

                if tiles.len() != figure.squares_entity.len()
                    || !figure.state_animation.is_default()
                {
                    next_state.set(StateGame::Idle);
                    event_writer.send(FigureDeniedPlacing(*placed));
                    info!("Invalid placement, returning to idle state.");
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

                next_state.set(StateGame::Placed(*placed));
            }
        }
    }
}
