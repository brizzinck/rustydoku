use bevy::prelude::*;

use crate::{
    components::figures::{dragging, placing, square::highlight, stop_dragging},
    resource::figure_spawner::spawn_figures,
};

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (dragging, stop_dragging, highlight));
        app.add_systems(PostUpdate, (placing, spawn_figures));
    }
}
