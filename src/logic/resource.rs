use crate::{
    constants::world::background::BACKGROUND_CLEAR_COLOR,
    resource::{
        figure_spawner::FigureSpawner, map::Map, music::MusicResource, score::Score,
        square::SquaresToDespawn,
    },
    states::gameplay::StateGame,
};
use bevy::prelude::*;

pub struct RustydokuResource;

impl RustydokuResource {
    pub fn init(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut next_state: ResMut<NextState<StateGame>>,
    ) {
        debug!("Initializing RustydokuResource");

        trace!("Adding Clear Color");
        commands.insert_resource(ClearColor(BACKGROUND_CLEAR_COLOR));

        trace!("Adding Score");
        commands.insert_resource(Score::default());

        trace!("Adding Map");
        commands.insert_resource(Map::default());

        trace!("Adding SquaresToDespawn");
        commands.insert_resource(SquaresToDespawn::default());

        trace!("Adding FigureSpawner");
        commands.insert_resource(FigureSpawner::init(&assets));

        trace!("Adding MusicResource");
        commands.insert_resource(MusicResource::init(&assets));

        debug!("RustydokuResource initialized");

        trace!("Setting next state to GenerateWorld");
        next_state.set(StateGame::GenerateWorld);
    }
}
