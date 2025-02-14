use crate::{
    components::ui::{
        button_restart::{handle_restart_button, spawn_button_restart_top},
        game_over::spawn_game_over_ui,
        score::{spawn_text_score, update_text_score},
    },
    states::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_text_score, spawn_button_restart_top));
        app.add_systems(Update, (update_text_score, handle_restart_button));
        app.add_systems(OnEnter(StateGame::GameOver), spawn_game_over_ui);
    }
}
