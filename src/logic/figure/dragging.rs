use crate::{
    components::figures::{Figure, FigureBounds},
    constants::figure::*,
    events::figure::FigureTriggerDragging,
    states::gameplay::StateGame,
    world::game_zone::GameZone,
};
use bevy::{prelude::*, window::PrimaryWindow};

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

impl Figure {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn moving(
        mut figure_query: Query<(&mut Transform, &Figure, &FigureBounds)>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        touch_input: Res<Touches>,
        cursor: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&Camera, &GlobalTransform)>,
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
                        if let Ok((mut transform, _, bounds)) = figure_query.get_mut(*figure) {
                            let desired = Figure::clamp_position(world_pos.origin, bounds);

                            transform.translation.x = desired.x;
                            transform.translation.y = desired.y;
                        }
                    }
                }
            }
        }
    }

    fn clamp_position(world_pos: Vec3, bounds: &FigureBounds) -> Vec3 {
        let min_offset = bounds.min * SQUARE_SIZE;
        let max_offset = bounds.max * SQUARE_SIZE;

        let min_x = GameZone::get().left_up.x - min_offset.x;
        let max_x = GameZone::get().right_down.x - max_offset.x;
        let min_y = GameZone::get().right_down.y - min_offset.y;
        let max_y = GameZone::get().left_up.y - max_offset.y;

        let mut desired = world_pos;
        desired.x = desired.x.clamp(min_x, max_x);
        desired.y = (desired.y
            + (FIGURE_OFFSET_DRAGGING_Y - bounds.min.y * FIGURE_OFFSET_DRAGGING_Y_MULTIPLY))
            .clamp(min_y, max_y);

        desired
    }
}
