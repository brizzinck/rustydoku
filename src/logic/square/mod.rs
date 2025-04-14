pub mod trigger;

use crate::{
    components::{
        figure::{square::SquareComponent, FigureComponent},
        world::map::TileComponent,
    },
    constants::map::TILE_SIZE,
    resource::figure_spawner::FigureSpawnerResource,
    states::gameplay::GameState,
};
use bevy::prelude::*;

impl SquareComponent {
    pub(crate) fn spawn_as_child(
        commands: &mut Commands,
        parent: Entity,
        position: Vec2,
        rotation: Quat,
        resource: &FigureSpawnerResource,
    ) -> Entity {
        let child = commands
            .spawn(Self::create_child(parent, position, rotation, resource))
            .set_parent(parent)
            .id();
        child
    }

    pub(crate) fn highlight(
        mut tile_query: Query<(&TileComponent, &mut Sprite, &GlobalTransform, Entity)>,
        figure_query: Query<&FigureComponent>,
        mut square_query: Query<(Entity, &GlobalTransform, &mut SquareComponent)>,
        current_state: Res<State<GameState>>,
        resource: Res<FigureSpawnerResource>,
    ) {
        for (tile, mut sprite, _, _) in tile_query.iter_mut() {
            sprite.image = tile.default_image.clone();
        }

        if let GameState::Dragging(figure) = current_state.get() {
            let mut highlight_tiles = vec![];
            if let Ok(figure) = figure_query.get(*figure) {
                if !figure.state_animation.is_default() {
                    return;
                }

                let tiles = tile_query
                    .iter()
                    .map(|(tile, _, transform, entity)| (tile, transform, entity))
                    .collect::<Vec<_>>();

                trace!("Figure squares len: {:?}", figure.squares_entity.len());

                trace!("Highlighting tiles len: {:?}", highlight_tiles.len());

                for &square_entity in figure.squares_entity.iter() {
                    if let Ok((_, square_transform, _)) = square_query.get_mut(square_entity) {
                        if let Some(tile_entity) = Self::check_for_place(square_transform, &tiles) {
                            highlight_tiles.push(tile_entity);
                        }
                    }
                }

                if highlight_tiles.len() == figure.squares_entity.len() {
                    for tile_entity in highlight_tiles.into_iter() {
                        if let Ok((_, mut sprite, _, _)) = tile_query.get_mut(tile_entity) {
                            sprite.image = resource.get_square_image_highlighted();
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn check_for_place(
        transofrm: &GlobalTransform,
        tile_query: &Vec<(&TileComponent, &GlobalTransform, Entity)>,
    ) -> Option<Entity> {
        let square_pos = transofrm.translation().truncate();
        let square_grid_x = (square_pos.x / TILE_SIZE).round() as i32;
        let square_grid_y = (square_pos.y / TILE_SIZE).round() as i32;

        for (tile, tile_transform, tile_entity) in tile_query.iter() {
            let tile_pos = tile_transform.translation().truncate();
            let tile_grid_x = (tile_pos.x / TILE_SIZE).round() as i32;
            let tile_grid_y = (tile_pos.y / TILE_SIZE).round() as i32;

            if (square_grid_x == tile_grid_x && square_grid_y == tile_grid_y)
                && tile.square.is_none()
            {
                return Some(*tile_entity);
            }
        }

        trace!(
            "No place for square found at {:?} | ({:?}, {:?})",
            square_pos,
            square_grid_x,
            square_grid_y
        );

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_for_place_found_works() {
        let square_translation = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
        let square_transform = GlobalTransform::from_translation(square_translation);

        let tile_translation = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
        let tile_transform = GlobalTransform::from_translation(tile_translation);

        let tile = TileComponent {
            square: None,
            default_image: Handle::default(),
        };

        let tile_entity = Entity::from_raw(42);

        let tiles = vec![(&tile, &tile_transform, tile_entity)];

        let result = SquareComponent::check_for_place(&square_transform, &tiles);

        assert_eq!(result, Some(tile_entity));
    }

    #[test]
    fn check_for_place_not_found_works() {
        let square_translation = Vec3::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0, 0.0);
        let square_transform = GlobalTransform::from_translation(square_translation);

        let tile_translation = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
        let tile_transform = GlobalTransform::from_translation(tile_translation);

        let tile = TileComponent {
            square: None,
            default_image: Handle::default(),
        };

        let tile_entity = Entity::from_raw(42);
        let tiles = vec![(&tile, &tile_transform, tile_entity)];

        let result = SquareComponent::check_for_place(&square_transform, &tiles);

        assert_eq!(result, None);
    }

    #[test]
    fn check_for_place_occupied_tile_works() {
        let square_translation = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
        let square_transform = GlobalTransform::from_translation(square_translation);

        let tile_translation = Vec3::new(TILE_SIZE, TILE_SIZE, 0.0);
        let tile_transform = GlobalTransform::from_translation(tile_translation);

        let tile = TileComponent {
            square: Some(Entity::from_raw(100)),
            default_image: Handle::default(),
        };

        let tile_entity = Entity::from_raw(42);
        let tiles = vec![(&tile, &tile_transform, tile_entity)];

        let result = SquareComponent::check_for_place(&square_transform, &tiles);

        assert_eq!(result, None);
    }
}
