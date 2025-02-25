use crate::{components::camera::RustyCamera2D, states::world::camera::StateCameraPosition};
use bevy::prelude::*;

impl RustyCamera2D {
    pub(crate) fn set_camera_game_over(mut next_state: ResMut<NextState<StateCameraPosition>>) {
        next_state.set(StateCameraPosition::ToGameOver);
    }

    pub(crate) fn set_camera_default(mut next_state: ResMut<NextState<StateCameraPosition>>) {
        next_state.set(StateCameraPosition::ToDefault);
    }
}
