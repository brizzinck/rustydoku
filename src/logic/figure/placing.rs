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
