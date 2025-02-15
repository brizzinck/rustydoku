use crate::{
    components::figures::{dragging, placing, square::highlight, stop_dragging},
    resource::figure_spawner::{
        despawn_figures, lerping_figures, spawn_figures, spawn_zone_figures,
    },
    states::StateGame,
};
use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct FigurePlugin;

impl Plugin for FigurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(StateGame::GenerateWorld),
            (spawn_zone_figures, spawn_figures).chain(),
        );
        app.add_systems(
            Update,
            (
                dragging,
                stop_dragging,
                highlight,
                placing,
                despawn_figures,
                lerping_figures,
            ),
        );
        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figures::FigureBounds;
            app.register_type::<FigureBounds>();
        }
    }
}
