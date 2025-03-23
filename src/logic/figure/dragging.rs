use crate::{
    components::figure::{FigureBoundsComponent, FigureComponent},
    constants::{figure::*, square::*},
    events::figure::FigureTriggerDraggingEvent,
    states::gameplay::GameState,
    world::game_zone::GameZone,
};
use bevy::{math::NormedVectorSpace, prelude::*, window::PrimaryWindow};

impl FigureComponent {
    /// System responsible for handling figure dragging when the game state is [`GameState::Dragging`].
    ///
    /// **Algorithm**:
    /// 1. Checks if the current game state is `GameState::Dragging`. If true, retrieves the specific [`Entity`] of the figure to be dragged.
    /// 2. Dispatches a [`FigureTriggerDraggingEvent`] to allow additional custom logic (e.g., animations, sound).
    /// 3. Retrieves the active camera and its transform.
    /// 4. Determines the current cursor position. Mouse input has priority if the left button is pressed;
    ///    otherwise, touch input is checked (first available touch).
    /// 5. Converts the cursor/touch position from viewport coordinates to world coordinates.
    /// 6. Clamps the resulting world position to keep it within valid bounds, taking into account the figure size
    ///    via [`Self::clamp_position`].
    /// 7. Updates the figure’s `Transform` (only `x` and `y`) to the clamped position.
    ///
    /// # Parameters
    /// - `figure_query`: Query granting access to [`Transform`], [`FigureComponent`], and [`FigureBoundsComponent`]
    ///   of the figure being dragged.
    /// - `mouse_input`: Resource representing the current mouse button states.
    /// - `touch_input`: Resource representing the current touch states.
    /// - `cursor`: Query to the primary window, used to retrieve the current cursor position.
    /// - `cameras`: Query to retrieve the active camera and its global transform.
    /// - `game_state`: Resource holding the current state of the game.
    /// - `event_writer`: Used to dispatch a [`FigureTriggerDraggingEvent`].
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

            let (position, offset) = if mouse_input.pressed(MouseButton::Left) {
                trace!("Mouse pressed");
                (
                    cursor.single().cursor_position(),
                    FIGURE_DRAG_OFFSET_Y_MOUSE,
                )
            } else {
                trace!("Touch pressed");
                (
                    touch_input.iter().next().map(|touch| touch.position()),
                    FIGURE_DRAG_OFFSET_Y_FINGER,
                )
            };

            trace!("Interaction position: {:?}", position);

            if let Some(cursor_pos) = position {
                if let Ok(world_pos) = camera.viewport_to_world(camera_transform, cursor_pos) {
                    if let Ok((mut transform, _, bounds)) = figure_query.get_mut(*figure) {
                        let desired =
                            FigureComponent::clamp_position(world_pos.origin, bounds, offset);

                        transform.translation.x = desired.x;
                        transform.translation.y = desired.y;
                    }
                }
            }
        }
    }

    /// Clamps the given world position so that the figure remains within the valid region of the game area.
    ///
    /// **How it works**:
    /// - Calculates the figure’s min/max offset by multiplying [`FigureBoundsComponent`] by `SQUARE_SIZE`.
    /// - Fetches the boundaries of the game zone from [`GameZone::get()`], typically specifying left-up and right-down corners.
    /// - Clamps `x` between `min_x` and `max_x` and `y` between `min_y` and `max_y`.
    /// - Additionally applies a vertical shift to `y` (using `offset * compute_offset_multiplier(...)`)
    ///   so that the figure is not completely covered by the finger or cursor.
    ///
    /// # Parameters
    /// - `world_pos`: The initial world position (e.g., converted from cursor coordinates).
    /// - `bounds`: The figure’s bounding box, used to calculate how much space it occupies.
    /// - `offset`: An extra vertical offset, e.g. [`FIGURE_DRAG_OFFSET_Y_MOUSE`] or [`FIGURE_DRAG_OFFSET_Y_FINGER`].
    ///
    /// # Returns
    /// A [`Vec3`] representing the clamped position. The `z` coordinate is taken from `world_pos` unchanged.
    fn clamp_position(world_pos: Vec3, bounds: &FigureBoundsComponent, offset: f32) -> Vec3 {
        let min_offset = bounds.min * SQUARE_SIZE;
        let max_offset = bounds.max * SQUARE_SIZE;

        // For a real project, ensure GameZone::get() is accessible or mocked in tests.
        let min_x = GameZone::get().left_up.x - min_offset.x;
        let max_x = GameZone::get().right_down.x - max_offset.x;
        let min_y = GameZone::get().right_down.y - min_offset.y;
        let max_y = GameZone::get().left_up.y - max_offset.y;

        let mut desired = world_pos;

        desired.x = desired.x.clamp(min_x, max_x);
        desired.y =
            (desired.y + offset * Self::compute_offset_multiplier(bounds)).clamp(min_y, max_y);

        trace!("Clamped position: {:?}", desired);

        desired
    }

    /// Computes an additional vertical offset multiplier so that the figure remains visible above the touch point.
    ///
    /// **Algorithm**:
    /// 1. Calculates the figure’s height along the Y-axis (distance between `bounds.min.y` and `bounds.max.y`).
    /// 2. If the figure’s height is less than [`FIGURE_DRAG_OFFSET_Y_MIN_MULTIPLIER`], returns that minimum
    ///    threshold value. This ensures very small figures (e.g. 1×1) are still moved visibly above the cursor/finger.
    /// 3. Otherwise, returns the figure’s actual height.
    fn compute_offset_multiplier(bounds: &FigureBoundsComponent) -> f32 {
        let height = bounds.max.y.distance(bounds.min.y);

        if height <= FIGURE_DRAG_OFFSET_Y_MIN_MULTIPLIER {
            FIGURE_DRAG_OFFSET_Y_MIN_MULTIPLIER
        } else {
            height
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::figure::FigureBoundsComponent;

    #[test]
    fn clamp_position_with_offset() {
        let bounds = FigureBoundsComponent {
            min: Vec2::new(0.5, 0.5),
            max: Vec2::new(1.0, 1.0),
        };

        let world_pos = Vec3::new(-2000., 1000., 1.);
        let offset = 50.;

        let result = FigureComponent::clamp_position(world_pos, &bounds, offset);

        let expected_result = Vec3::new(-180., 120., 1.);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn compute_offset_multiplier_small_works() {
        let small_bounds = FigureBoundsComponent {
            min: Vec2::new(0.0, 0.0),
            max: Vec2::new(0.5, 0.5),
        };

        let multiplier_small = FigureComponent::compute_offset_multiplier(&small_bounds);

        assert_eq!(multiplier_small, FIGURE_DRAG_OFFSET_Y_MIN_MULTIPLIER,);
    }

    #[test]
    fn compute_offset_multiplier_big_works() {
        let bigger_bounds = FigureBoundsComponent {
            min: Vec2::new(0., 0.),
            max: Vec2::new(2., 3.),
        };

        let multiplier_big = FigureComponent::compute_offset_multiplier(&bigger_bounds);

        assert_eq!(multiplier_big, 3.);
    }
}
