use crate::components::ui::{
    button_restart::handle_restart_button,
    header::{score::update_text_score, spawn_header},
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_header);
        app.add_systems(Update, (update_text_score, handle_restart_button));
    }
}
