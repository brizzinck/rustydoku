use crate::{
    components::figure::{square::SquareComponent, FigureComponent},
    events::figure::*,
    states::gameplay::GameState,
};
use bevy::prelude::*;

#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

pub struct RustydokuFigurePlugin;

impl Plugin for RustydokuFigurePlugin {
    fn build(&self, app: &mut App) {
        debug!("Building RustydokuFigurePlugin");

        trace!("Adding event FigureTriggerDragging");
        app.add_event::<FigureTriggerDraggingEvent>();

        trace!("Adding event FigureTriggerUp");
        app.add_event::<FigureTriggerUpEvent>();

        trace!("Adding event FigureDeniedPlacing");
        app.add_event::<FigureDeniedPlacingEvent>();

        trace!("Adding event FigureSpawned");
        app.add_event::<FigureSpawnedEvent>();

        trace!("Adding event FigureCantPlaced");
        app.add_event::<FigureCantPlacedEvent>();

        trace!("Adding event FigureCanPlaced");
        app.add_event::<FigureCanPlacedEvent>();

        trace!("Adding systems to RustydokuFigurePlugin when dragging event");
        app.add_systems(
            Update,
            (
                FigureComponent::stop_dragging,
                FigureComponent::dragging,
                SquareComponent::highlight,
                FigureComponent::call_dragging_events,
            )
                .run_if(GameState::when_draggin)
                .chain(),
        );

        trace!("Adding systems to RustydokuFigurePlugin update");
        app.add_systems(Update, SquareComponent::call_despawn);

        #[cfg(feature = "debug-inspector")]
        {
            use crate::components::figure::FigureBoundsComponent;
            app.register_type::<FigureBounds>();
        }

        debug!("RustydokuFigurePlugin built");
    }
}
