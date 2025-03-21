use crate::{
    components::world::camera::RustyCamera2DComponent, states::world::camera::CameraPositionState,
};
use bevy::prelude::*;

impl RustyCamera2DComponent {
    pub(crate) fn set_camera_game_over(mut next_state: ResMut<NextState<CameraPositionState>>) {
        next_state.set(CameraPositionState::ToGameOver);
        trace!("Next state StateCameraPosition::ToGameOver");
    }

    pub(crate) fn set_camera_default(mut next_state: ResMut<NextState<CameraPositionState>>) {
        next_state.set(CameraPositionState::ToDefault);
        trace!("Next state StateCameraPosition::ToDefault");
    }
}
