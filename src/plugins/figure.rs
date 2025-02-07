use bevy::prelude::*;

use crate::components::figures::{
    drag_figure,
    spawner::spawn_figures,
    square::{highlight_tile, place},
    stop_dragging,
};

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_figures);
        app.add_systems(Update, (drag_figure, stop_dragging, highlight_tile));
        app.add_systems(PostUpdate, place);
    }
}
