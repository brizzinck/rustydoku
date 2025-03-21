use crate::{
    components::figure::{FigureBoundsComponent, FigureComponent},
    constants::{figure::*, square::*},
    events::figure::FigureTriggerDraggingEvent,
    states::gameplay::GameState,
    world::game_zone::GameZone,
};
use bevy::{prelude::*, window::PrimaryWindow};

impl FigureComponent {
    /// Handles dragging of a figure when the game is in the `Dragging` state.
    ///
    /// This system performs the following steps:
    /// - Checks if the current game state is `GameState::Dragging` and retrieves the figure to be dragged.
    /// - Sends a `FigureTriggerDraggingEvent` for additional processing.
    /// - Retrieves the primary camera and its transform.
    /// - Determines the current cursor position, prioritizing mouse input (if the left button is pressed)
    ///   and falling back to touch input.
    /// - Converts the cursor's viewport position to world coordinates.
    /// - Clamps the calculated world position within the allowed bounds for the figure using
    ///   the helper function [`clamp_position`].
    /// - Updates the figure's transform with the clamped position.
    ///
    /// # Parameters
    ///
    /// - `figure_query`: A query for mutable references to a figure’s [`Transform`], [`FigureComponent`],
    ///   and [`FigureBoundsComponent`].
    /// - `mouse_input`: A resource providing the current state of mouse buttons.
    /// - `touch_input`: A resource providing the current touch input data.
    /// - `cursor`: A query for the primary window, used to obtain the current cursor position.
    /// - `cameras`: A query to access the active camera and its global transform.
    /// - `game_state`: A resource representing the current state of the game.
    /// - `event_writer`: An event writer to dispatch a [`FigureTriggerDraggingEvent`] when dragging occurs.
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

            // Determine the input position: use mouse input if the left button is pressed,
            // otherwise fall back to touch input.
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

    /// Clamps a given world position to ensure it stays within the allowed bounds of the game zone.
    ///
    /// This helper function calculates the minimum and maximum allowable offsets based on the figure's bounds
    /// (scaled by `SQUARE_SIZE`) and the boundaries of the current game zone (retrieved via [`GameZone::get`]).
    /// It then clamps the `x` coordinate between the computed `min_x` and `max_x`, and similarly for the `y` coordinate,
    /// while also applying an additional vertical offset calculated from `FIGURE_DRAG_OFFSET_Y` and
    /// `FIGURE_DRAG_OFFSET_Y_MULTIPLIER`.
    ///
    /// # Parameters
    ///
    /// - `world_pos`: The original position in world coordinates.
    /// - `bounds`: A reference to the figure's [`FigureBoundsComponent`] that defines its constaints border figure.
    ///
    /// # Returns
    ///
    /// A new [`Vec3`] representing the clamped position within the allowed boundaries.
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

    /// Tests that the `clamp_position` function correctly clamps a world position within the given bounds.
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
