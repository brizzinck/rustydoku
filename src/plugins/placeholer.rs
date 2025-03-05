use crate::{
    components::world::placeholder::Placeholder,
    states::{figure::placeholder::StatePlaceholderAnimation, gameplay::StateGame},
};
use bevy::prelude::*;

pub struct PlaceholderPlugin;

impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Placeholder::bouncing_init.run_if(StatePlaceholderAnimation::when_bouncing_init),
        );

        app.add_systems(
            Update,
            Placeholder::bouncing_default.run_if(StatePlaceholderAnimation::when_bouncing_default),
        );

        app.add_systems(
            Update,
            Placeholder::bouncing_peak.run_if(StatePlaceholderAnimation::when_bouncing_peak),
        );

        app.add_systems(OnExit(StateGame::CheckGameOver), Placeholder::update_image);
    }
}
