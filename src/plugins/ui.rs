use crate::{
    components::ui::{
        button_restart::handle_restart_button, game_over::spawn_game_over_ui, header::spawn_header,
        score::update_text_score,
    },
    states::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuUIPlugin;

impl Plugin for RustydokuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_header);
        app.add_systems(Update, (update_text_score, handle_restart_button));
        app.add_systems(OnEnter(StateGame::GameOver), spawn_game_over_ui);
    }
}
