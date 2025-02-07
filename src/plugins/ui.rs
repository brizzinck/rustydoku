use crate::components::ui::{
    button_restart::{handle_restart_button, spawn_button_restart},
    score::{spawn_text_score, update_text_score},
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_text_score, spawn_button_restart)); // Create UI
        app.add_systems(Update, (update_text_score, handle_restart_button)); // Update score text when score changes
    }
}
