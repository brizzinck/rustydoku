use super::map::Tile;
use crate::{
    constants::figure::*,
    events::figure::{FigureTriggerDragging, FigureTriggerUp},
    resource::{figure_spawner::FigureSpawner, game_zone::GameZone},
    states::gameplay::StateGame,
};
use bevy::{prelude::*, window::PrimaryWindow};
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;
use square::{check_for_place, Square};

pub mod animation;
pub mod big_t_shape;
pub mod cube;
pub mod line;
pub mod spawner;
pub mod square;
pub mod t_shape;

#[derive(Component, Debug, Copy, Clone)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct FigureBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Clone, Component, Default, Debug)]
pub struct Figure {
    pub squares_entity: Vec<Entity>,
    pub squares_position: Vec<Vec2>,
}

pub(crate) fn start_dragging(
    trigger: Trigger<Pointer<Down>>,
    figures: Query<Entity, With<Figure>>,
    square_query: Query<&Square>,
    current_state: Res<State<StateGame>>,
    mut state_figure: ResMut<NextState<StateGame>>,
) {
    if *current_state.get() != StateGame::Idle {
        return;
    }

    if let Ok(square) = square_query.get(trigger.target) {
        if let Some(parent) = square.parent {
            if let Ok(entity) = figures.get(parent) {
                state_figure.set(StateGame::Dragging(entity));
            }
        }
    } else if let Ok(entity) = figures.get(trigger.target) {
        state_figure.set(StateGame::Dragging(entity));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn moving(
    mut figure_query: Query<(&mut Transform, &Figure, &FigureBounds)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    cursor: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    game_zone: Res<GameZone>,
    game_state: Res<State<StateGame>>,
    mut event_writer: EventWriter<FigureTriggerDragging>,
) {
    if let StateGame::Dragging(figure) = game_state.get() {
        event_writer.send(FigureTriggerDragging(*figure));
        let (camera, camera_transform) = cameras.single();

        if mouse_input.pressed(MouseButton::Left) || touch_input.any_just_pressed() {
            let window = cursor.single();
            if let Some(cursor_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    let mut desired = world_pos.origin;

                    if let Ok((mut transform, _, bounds)) = figure_query.get_mut(*figure) {
                        let min_offset = bounds.min * SQUARE_SIZE;
                        let max_offset = bounds.max * SQUARE_SIZE;

                        let min_x = game_zone.left_up.x - min_offset.x;
                        let max_x = game_zone.right_down.x - max_offset.x;
                        let min_y = game_zone.right_down.y - min_offset.y;
                        let max_y = game_zone.left_up.y - max_offset.y;

                        desired.x = desired.x.clamp(min_x, max_x);
                        desired.y = (desired.y
                            + (FIGURE_OFFSET_DRAGGING_Y
                                - bounds.min.y * FIGURE_OFFSET_DRAGGING_Y_MULTIPLY))
                            .clamp(min_y, max_y);

                        transform.translation.x = desired.x;
                        transform.translation.y = desired.y;
                    }
                }
            }
        }
    }
}

pub(crate) fn stop_dragging(
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    current_state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
    mut event_writer: EventWriter<FigureTriggerUp>,
) {
    if mouse_input.just_released(MouseButton::Left) || touch_input.any_just_released() {
        if let StateGame::Dragging(figure) = current_state.get() {
            next_state.set(StateGame::Placing(*figure));
            event_writer.send(FigureTriggerUp(*figure));
        }
    }
}

pub(crate) fn placing(
    mut commands: Commands,
    current_state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
    mut square_query: Query<(Entity, &GlobalTransform, &mut Transform)>,
    mut tile_query: Query<(&mut Tile, &GlobalTransform, Entity, &mut Sprite)>,
    figure_query: Query<&mut Figure>,
    mut figure_spawner: ResMut<FigureSpawner>,
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
                    if let Some(entity) = check_for_place(transform, &all_tiles) {
                        tiles.push(entity);
                    }
                }
            }

            if tiles.len() != figure.squares_entity.len() {
                next_state.set(StateGame::Idle);
                figure_spawner.add_lerp_figure(*placed);
                info!("Invalid placement, returning to idle state.");
                return;
            }

            for &square in &figure.squares_entity {
                if let Ok((square_entity, _, mut square_local_transform)) =
                    square_query.get_mut(square)
                {
                    commands.entity(square_entity).remove_parent();

                    if let Some(tile_entity) = tiles.pop() {
                        if let Ok((mut tile, _, _, mut sprite)) = tile_query.get_mut(tile_entity) {
                            tile.square = Some(square_entity);
                            sprite.color = tile.default_color;

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
