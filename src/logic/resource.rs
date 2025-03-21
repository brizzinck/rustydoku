use crate::{
    constants::world::background::BACKGROUND_CLEAR_COLOR,
    resource::{
        audio::RustydokuAudioResource, figure_spawner::FigureSpawnerResource, map::MapComponent,
        score::ScoreResource, square::SquaresToDespawnResource,
    },
    states::gameplay::GameState,
};
use bevy::prelude::*;

pub struct RustydokuResource;

impl RustydokuResource {
    pub fn init(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        debug!("Initializing RustydokuResource");

        trace!("Adding Clear Color");
        commands.insert_resource(ClearColor(BACKGROUND_CLEAR_COLOR));

        trace!("Adding Score");
        commands.insert_resource(ScoreResource::default());

        trace!("Adding Map");
        commands.insert_resource(MapComponent::default());

        trace!("Adding SquaresToDespawn");
        commands.insert_resource(SquaresToDespawnResource::default());

        trace!("Adding FigureSpawner");
        commands.insert_resource(FigureSpawnerResource::init(&assets));

        trace!("Adding MusicResource");
        commands.insert_resource(RustydokuAudioResource::init(&assets));

        debug!("RustydokuResource initialized");

        trace!("Setting next state to GenerateWorld");
        next_state.set(GameState::GenerateWorld);
    }
}
