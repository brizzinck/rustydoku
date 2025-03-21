use crate::{
    components::figure::{FigureBoundsComponent, FigureComponent},
    constants::{figure::*, square::*},
    events::figure::FigureTriggerDraggingEvent,
    states::gameplay::GameState,
    world::game_zone::GameZone,
};
use bevy::{prelude::*, window::PrimaryWindow};

impl FigureComponent {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn dragging(
        mut figure_query: Query<(&mut Transform, &FigureComponent, &FigureBoundsComponent)>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        touch_input: Res<Touches>,
        cursor: Query<&Window, With<PrimaryWindow>>,
        cameras: Query<(&Camera, &GlobalTransform)>,
        game_state: Res<State<GameState>>,
        mut event_writer: EventWriter<FigureTriggerDraggingEvent>,
    ) {
        if let GameState::Dragging(figure) = game_state.get() {
            trace!("Dragging figure: {:?}", figure);
            event_writer.send(FigureTriggerDraggingEvent(*figure));
            let (camera, camera_transform) = cameras.single();

            let position = if mouse_input.pressed(MouseButton::Left) {
                trace!("Mouse pressed");
                cursor.single().cursor_position()
            } else {
                trace!("Touch pressed");
                touch_input.iter().next().map(|touch| touch.position())
            };

            trace!("Interaction position: {:?}", position);

            if let Some(cursor_pos) = position {
                if let Ok(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    if let Ok((mut transform, _, bounds)) = figure_query.get_mut(*figure) {
                        let desired = FigureComponent::clamp_position(world_pos.origin, bounds);

                        transform.translation.x = desired.x;
                        transform.translation.y = desired.y;
                    }
                }
            }
        }
    }

    fn clamp_position(world_pos: Vec3, bounds: &FigureBoundsComponent) -> Vec3 {
        let min_offset = bounds.min * SQUARE_SIZE;
        let max_offset = bounds.max * SQUARE_SIZE;

        let min_x = GameZone::get().left_up.x - min_offset.x;
        let max_x = GameZone::get().right_down.x - max_offset.x;
        let min_y = GameZone::get().right_down.y - min_offset.y;
        let max_y = GameZone::get().left_up.y - max_offset.y;

        let mut desired = world_pos;
        desired.x = desired.x.clamp(min_x, max_x);
        desired.y = (desired.y
            + (FIGURE_DRAG_OFFSET_Y - bounds.min.y * FIGURE_DRAG_OFFSET_Y_MULTIPLIER))
            .clamp(min_y, max_y);

        trace!("Clamped position: {:?}", desired);

        desired
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::figure::FigureBoundsComponent;

    #[test]
    fn clamp_position_works() {
        let bounds = FigureBoundsComponent {
            min: Vec2::new(1.0, 1.0),
            max: Vec2::new(2.0, 2.0),
        };

        let world_pos = Vec3::new(-2000., 1000., 1.);

        let result = FigureComponent::clamp_position(world_pos, &bounds);
        let expected_result = Vec3::new(-200.0, 80.0, 1.0);

        assert_eq!(result, expected_result);
    }
}
