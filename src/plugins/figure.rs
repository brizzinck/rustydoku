use bevy::prelude::*;

use crate::{
    components::figures::{
        drag_figure,
        square::{highlight_tile, place},
        stop_dragging,
    },
    resource::figure_spawner::spawn_figures,
};

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (drag_figure, stop_dragging, highlight_tile));
        app.add_systems(PostUpdate, (place, spawn_figures));
    }
}
