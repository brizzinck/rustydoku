use crate::{
    components::world::placeholder::PlaceholderComponent,
    states::{figure::placeholder::PlaceholderAnimationState, gameplay::GameState},
};
use bevy::prelude::*;

pub struct RustydokuPlaceholderPlugin;

impl Plugin for RustydokuPlaceholderPlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuPlaceholderPlugin");

        trace!("Adding systems to RustydokuPlaceholderPlugin update when bouncing init");
        app.add_systems(
            Update,
            PlaceholderComponent::bouncing_init
                .run_if(PlaceholderAnimationState::when_bouncing_init),
        );

        trace!("Adding systems to RustydokuPlaceholderPlugin update when bouncing default");
        app.add_systems(
            Update,
            PlaceholderComponent::bouncing_default
                .run_if(PlaceholderAnimationState::when_bouncing_default),
        );

        trace!("Adding systems to RustydokuPlaceholderPlugin update when bouncing peak");
        app.add_systems(
            Update,
            PlaceholderComponent::bouncing_peak
                .run_if(PlaceholderAnimationState::when_bouncing_peak),
        );

        trace!("Adding systems to RustydokuPlaceholderPlugin on state exit game over");
        app.add_systems(
            OnExit(GameState::CheckGameOver),
            PlaceholderComponent::update_image,
        );

        debug!("Finished building RustydokuPlaceholderPlugin");
    }
}
