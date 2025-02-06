use bevy::{prelude::*, window::PrimaryWindow};

pub mod cube;
pub mod square;

#[derive(Copy, Clone, Component)]
pub enum FigureType {
    Cube,
}

#[derive(Copy, Clone, Component, Default, Debug)]
pub struct Figure {
    pub is_dragging: bool,
}

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, cube::spawn);
        app.add_systems(Update, (drag_figure, stop_dragging, square::hightligh_tile));
    }
}

pub(super) fn start_dragging(_: Trigger<Pointer<Down>>, mut cube: Query<&mut Figure>) {
    cube.single_mut().is_dragging = true;
}

fn drag_figure(
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
fn stop_dragging(
    mut query: Query<&mut Figure>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
) {
    if mouse_input.just_released(MouseButton::Left) || touch_input.any_just_released() {
        for mut draggable in &mut query {
            draggable.is_dragging = false;
        }
    }
}
