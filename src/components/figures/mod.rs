use bevy::{prelude::*, window::PrimaryWindow};
use square::Square;

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
    mut query: Query<(&mut Transform, &Figure)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    cursor: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = cameras.single();

    if mouse_button_input.pressed(MouseButton::Left) || touch_input.any_just_pressed() {
        let cursor = cursor.single().cursor_position();

        if let Some(cursor_pos) = cursor {
            if let Ok(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let cursor_world_pos = world_pos.origin;
                for (mut transform, draggable) in &mut query {
                    if draggable.is_dragging {
                        transform.translation = Vec3::new(
                            cursor_world_pos.x,
                            cursor_world_pos.y + 50.,
                            transform.translation.z,
                        );
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
