use crate::{
    components::figure::{square::Square, Figure},
    states::gameplay::StateGame,
};
use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct RustydokuFigurePlugin;

impl Plugin for RustydokuFigurePlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuFigurePlugin");

        trace!("Adding systems to RustydokuFigurePlugin when dragging event");
        app.add_systems(
            Update,
            (
                Figure::stop_dragging,
                Figure::dragging,
                Square::highlight,
                Figure::call_dragging_events,
            )
                .run_if(StateGame::when_draggin)
                .chain(),
        );

        trace!("Adding systems to RustydokuFigurePlugin update");
        app.add_systems(Update, Square::call_despawn);

        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figure::FigureBounds;
            app.register_type::<FigureBounds>();
        }

        debug!("RustydokuFigurePlugin built");
    }
}
