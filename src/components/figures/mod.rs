use bevy::{prelude::*, window::PrimaryWindow};
use square::Square;

use crate::resource::game_zone::GameZone;

use super::map::TILE_SIZE;

pub mod big_t_shape;
pub mod cube;
pub mod spawner;
pub mod square;
pub mod t_shape;

#[derive(Copy, Clone, Component)]
pub enum FigureType {
    Cube,
    TShape,
    Square,
    BigTShape,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct FigureBounds {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Copy, Clone, Component, Default, Debug)]
pub struct Figure {
    pub is_dragging: bool,
}

pub(crate) fn start_dragging(
    trigger: Trigger<Pointer<Down>>,
    mut cubes: Query<(&mut Figure, Option<&Children>)>,
    mut square_query: Query<(&Parent, &mut Square)>,
) {
    if let Ok((parent, _)) = square_query.get(trigger.target) {
        if let Ok((mut figure, children)) = cubes.get_mut(parent.get()) {
            figure.is_dragging = true;

            if let Some(children) = children {
                for &child in children.iter() {
                    if let Ok((_, mut square)) = square_query.get_mut(child) {
                        square.state = square::SquareState::Dragging;
                    }
                }
            }
        }
    }
}

pub(crate) fn drag_figure(
    mut figure_query: Query<(&mut Transform, &Figure, &FigureBounds)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    cursor: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    game_zone: Res<GameZone>,
) {
    let (camera, camera_transform) = cameras.single();

    if mouse_button_input.pressed(MouseButton::Left) || touch_input.any_just_pressed() {
        let window = cursor.single();
        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let mut desired = world_pos.origin;

                for (mut transform, figure, bounds) in &mut figure_query {
                    if figure.is_dragging {
                        let min_offset = bounds.min * TILE_SIZE;
                        let max_offset = bounds.max * TILE_SIZE;

                        let min_x = game_zone.left_up.x - min_offset.x;
                        let max_x = game_zone.right_down.x - max_offset.x;
                        let min_y = game_zone.right_down.y - min_offset.y;
                        let max_y = game_zone.left_up.y - max_offset.y;

                        desired.x = desired.x.clamp(min_x, max_x);
                        desired.y = desired.y.clamp(min_y, max_y);

                        transform.translation.x = desired.x;
                        transform.translation.y = desired.y;

                        break;
                    }
                }
            }
        }
    }
}

pub(crate) fn stop_dragging(
    mut query: Query<(&mut Figure, Option<&Children>)>,
    mut square_query: Query<&mut Square>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) {
    if mouse_input.just_released(MouseButton::Left) || touch_input.any_just_released() {
        for (mut draggable, children) in &mut query {
            draggable.is_dragging = false;

            if let Some(children) = children {
                for &child in children.iter() {
                    if let Ok(mut square) = square_query.get_mut(child) {
                        if let square::SquareState::Placing(pos) = square.state {
                            square.state = square::SquareState::MustBePlaced(pos);
                        }
                    }
                }
            }
        }
    }
}
