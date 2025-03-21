use crate::{
    components::{
        figure::{FigureBoundsComponent, FigureComponent},
        world::map::TileComponent,
    },
    constants::map::{MAP_SPAWN_POSITIOM, TILE_SIZE},
    events::figure::{FigureCanPlacedEvent, FigureCantPlacedEvent},
    resource::map::MapComponent,
    states::gameplay::GameState,
    world::gameplay::Gameplay,
};
use bevy::prelude::*;

impl Gameplay {
    pub(crate) fn check_game_over(
        figures: Query<(&Transform, &FigureBoundsComponent, &FigureComponent)>,
        map: Res<MapComponent>,
        tiles: Query<&TileComponent>,
        mut next_state: ResMut<NextState<GameState>>,
        mut cant_place_writer: EventWriter<FigureCantPlacedEvent>,
        mut can_place_writer: EventWriter<FigureCanPlacedEvent>,
    ) {
        let mut game_over = true;

        for (transform, bounds, figure) in figures.iter() {
            trace!(
                "Checking figure {:?} with bounds.min={:?}, squares_offset={:?}",
                transform.translation,
                bounds.min,
                figure.squares_position
            );

            let mut figure_cant_placed = true;
            'outer: for grid_x in MAP_SPAWN_POSITIOM {
                for grid_y in MAP_SPAWN_POSITIOM {
                    let grid = Vec2::new(grid_x as f32, grid_y as f32);

                    if Self::can_place_figure_at_grid(
                        grid,
                        bounds.min,
                        &figure.squares_position,
                        transform,
                        &map,
                        &tiles,
                    ) {
                        trace!(
                        "Found a valid placement for figure at grid=({}, {}) => Game continues!",
                        grid_x,
                        grid_y
                    );

                        figure_cant_placed = false;
                        game_over = false;

                        break 'outer;
                    }
                }
            }

            if figure_cant_placed {
                cant_place_writer.send(FigureCantPlacedEvent(figure.placeholder));
            } else {
                can_place_writer.send(FigureCanPlacedEvent(figure.placeholder));
            }
        }

        if game_over {
            next_state.set(GameState::GameOver);
            trace!("Next state is set to StateGame::GameOver");
        } else {
            next_state.set(GameState::Idle);
            trace!("Next state is set to StateGame::Idle");
        }
    }

    fn can_place_figure_at_grid(
        grid: Vec2,
        bounds_min: Vec2,
        offsets: &[Vec2],
        figure_transform: &Transform,
        map: &MapComponent,
        tiles: &Query<&TileComponent>,
    ) -> bool {
        let placement_translation = grid * TILE_SIZE - bounds_min * TILE_SIZE;

        trace!(
            "Trying grid=({:.0}, {:.0}); computed placement_translation={:?}",
            grid.x,
            grid.y,
            placement_translation
        );

        for offset in offsets {
            let rotated_offset = figure_transform.rotation * offset.extend(0.0);
            let candidate_pos = placement_translation + rotated_offset.truncate() * TILE_SIZE;

            trace!(
                " -> Checking square offset={:?}; rotated_offset={:?}; candidate_pos={:?}",
                offset,
                rotated_offset,
                candidate_pos
            );

            if Self::correct_to_place(candidate_pos.extend(0.0), map, tiles).is_none() {
                trace!("Cannot place at {:?}", candidate_pos);
                return false;
            }

            trace!("Square can be placed at {:?}", candidate_pos);
        }

        trace!("All squares OK at grid=({:.0}, {:.0})", grid.x, grid.y);

        true
    }

    fn correct_to_place(
        pos: Vec3,
        map: &MapComponent,
        tiles: &Query<&TileComponent>,
    ) -> Option<(i32, i32)> {
        let tile_coords = (
            (pos.x / TILE_SIZE).round() as i32,
            (pos.y / TILE_SIZE).round() as i32,
        );

        if let Some(tile_entity) = map.get(tile_coords) {
            if let Ok(tile) = tiles.get(*tile_entity) {
                if tile.square.is_none() {
                    return Some(tile_coords);
                } else {
                    trace!("Tile at {:?} is occupied by {:?}", tile_coords, tile.square);
                }
            } else {
                trace!("Could not query tile at {:?}", tile_coords);
            }
        } else {
            trace!("No tile found in the map for {:?}", tile_coords);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, Resource)]
    struct CanPlaceResult(bool);

    fn call_can_place(
        tiles: Query<&TileComponent>,
        map: Res<MapComponent>,
        mut result: ResMut<CanPlaceResult>,
    ) {
        let grid = Vec2::new(1.0, 1.0);
        let bounds_min = Vec2::ZERO;
        let offsets = &[Vec2::ZERO];
        let figure_transform = Transform::default();
        result.0 = Gameplay::can_place_figure_at_grid(
            grid,
            bounds_min,
            offsets,
            &figure_transform,
            &*map,
            &tiles,
        );
    }

    #[test]
    fn can_place_figure_at_grid_success() {
        let mut app = App::new();
        app.insert_resource(MapComponent::default());
        app.insert_resource(CanPlaceResult::default());

        let world = app.world_mut();
        let tile_entity = world
            .spawn(())
            .insert(TileComponent {
                square: None,
                default_image: Handle::default(),
            })
            .id();

        let mut map = world.resource_mut::<MapComponent>();
        map.0.insert((1, 1), tile_entity);

        app.add_systems(Update, call_can_place);

        app.update();

        let result = app.world().resource::<CanPlaceResult>().0;
        let expected_result = true;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_place_figure_at_grid_failure_due_to_occupied_tile() {
        let mut app = App::new();
        app.insert_resource(MapComponent::default());
        app.insert_resource(CanPlaceResult::default());

        let world = app.world_mut();
        let tile_entity = world
            .spawn(())
            .insert(TileComponent {
                square: Some(Entity::from_raw(42)),
                default_image: Handle::default(),
            })
            .id();

        let mut map = world.resource_mut::<MapComponent>();
        map.0.insert((1, 1), tile_entity);

        app.add_systems(Update, call_can_place);

        app.update();

        let result = app.world().resource::<CanPlaceResult>().0;
        let expected_result = false;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_place_figure_at_grid_failure_due_to_missing_tile() {
        let mut app = App::new();
        app.insert_resource(MapComponent::default());
        app.insert_resource(CanPlaceResult::default());

        app.add_systems(Update, call_can_place);
        app.update();

        let result = app.world().resource::<CanPlaceResult>().0;
        let expected_result = false;

        assert_eq!(result, expected_result);
    }
}
