use crate::{
    components::world::placeholder::Placeholder,
    states::{figure::placeholder::StatePlaceholderAnimation, gameplay::StateGame},
};
use bevy::prelude::*;

pub struct RustydokuPlaceholderPlugin;

impl Plugin for RustydokuPlaceholderPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuPlaceholderPlugin");

        trace!("Adding systems to RustydokuPlaceholderPlugin update when bouncing init");
        app.add_systems(
            Update,
            Placeholder::bouncing_init.run_if(StatePlaceholderAnimation::when_bouncing_init),
        );

        trace!("Adding systems to RustydokuPlaceholderPlugin update when bouncing default");
        app.add_systems(
            Update,
            Placeholder::bouncing_default.run_if(StatePlaceholderAnimation::when_bouncing_default),
        );

        trace!("Adding systems to RustydokuPlaceholderPlugin update when bouncing peak");
        app.add_systems(
            Update,
            Placeholder::bouncing_peak.run_if(StatePlaceholderAnimation::when_bouncing_peak),
        );

        trace!("Adding systems to RustydokuPlaceholderPlugin on state exit game over");
        app.add_systems(OnExit(StateGame::CheckGameOver), Placeholder::update_image);

        debug!("Finished building RustydokuPlaceholderPlugin");
    }
}
