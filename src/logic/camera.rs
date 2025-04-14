use crate::{
    components::world::camera::RustyCamera2DComponent, states::world::camera::CameraPositionState,
};
use bevy::prelude::*;

impl RustyCamera2DComponent {
    /// Triggers the camera transition to the **Game Over** position.
    ///
    /// This function updates the [`CameraPositionState`] to [`CameraPositionState::ToGameOver`],
    /// signaling systems responsible for animation to move the camera.
    ///
    /// # Parameters
    /// - `next_state`: A mutable reference to the [`NextState<CameraPositionState>`] resource.
    pub(crate) fn set_camera_game_over(mut next_state: ResMut<NextState<CameraPositionState>>) {
        next_state.set(CameraPositionState::ToGameOver);
        trace!("Next state StateCameraPosition::ToGameOver");
    }

    /// Triggers the camera transition back to its **default** position.
    ///
    /// This function updates the [`CameraPositionState`] to [`CameraPositionState::ToDefault`],
    /// instructing camera systems to reset the camera view (e.g. restarting the game).
    ///
    /// # Parameters
    /// - `next_state`: A mutable reference to the [`NextState<CameraPositionState>`] resource.
    pub(crate) fn set_camera_default(mut next_state: ResMut<NextState<CameraPositionState>>) {
        next_state.set(CameraPositionState::ToDefault);
        trace!("Next state StateCameraPosition::ToDefault");
    }

    /// Spawns the main 2D camera used for gameplay.
    ///
    /// # Parameters
    /// - `commands`: Bevy commands used to spawn the camera entity.
    pub(crate) fn spawn(mut commands: Commands) {
        commands.spawn((Camera2d, RustyCamera2DComponent));
    }
}
