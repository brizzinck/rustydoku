use crate::{
    constants::world::background::BACKGROUND_CLEAR_COLOR,
    resource::{figure_spawner::FigureSpawner, map::Map, score::Score, square::SquaresToDespawn},
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
        commands.insert_resource(ClearColor(BACKGROUND_CLEAR_COLOR));
        commands.insert_resource(Score::default());
        commands.insert_resource(FigureSpawner::default());
        commands.insert_resource(Map::default());
        commands.insert_resource(SquaresToDespawn::default());
        commands.insert_resource(FigureSpawner::init(&assets));

        next_state.set(StateGame::GenerateWorld);
    }
}
