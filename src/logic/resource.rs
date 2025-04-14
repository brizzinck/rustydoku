use crate::{
    constants::world::background::BACKGROUND_CLEAR_COLOR,
    resource::{
        audio::RustydokuAudioResource, figure_spawner::FigureSpawnerResource, map::MapComponent,
        score::ScoreResource, square::SquaresToDespawnResource,
    },
    states::gameplay::GameState,
};
use bevy::prelude::*;

/// A helper struct used for initializing core game resources at the start of the game.
///
/// This struct is not stored as a resource itself but provides the `init` method to set up:
/// - Background color
/// - Score tracking
/// - Map entity storage
/// - Despawn animation queue
/// - Figure spawning logic
/// - Audio system and assets
///
/// It also transitions the game state to [`GameState::GenerateWorld`] once setup is complete.
pub struct RustydokuResource;

impl RustydokuResource {
    /// Initializes all required game resources and transitions to the `GenerateWorld` state.
    ///
    /// This function is typically run during the [`GameState::InitResources`] state.
    /// It inserts the following resources into the ECS world:
    ///
    /// - [`ClearColor`]: Sets the background color of the game.
    /// - [`ScoreResource`]: Tracks the current and maximum score.
    /// - [`MapComponent`]: Stores the logical mapping of entities on the board.
    /// - [`SquaresToDespawnResource`]: Queues square entities for animated despawning.
    /// - [`FigureSpawnerResource`]: Manages figure placement, upscaling, and lerping animations.
    /// - [`RustydokuAudioResource`]: Manages audio clips and volume state.
    ///
    /// Finally, it sets the next [`GameState`] to [`GameState::GenerateWorld`] to begin
    /// world generation.
    ///
    /// # Parameters
    /// - `commands`: Bevy's [`Commands`] used to insert resources.
    /// - `assets`: A handle to the [`AssetServer`] used for loading assets like images and audio.
    /// - `next_state`: Mutable reference to the [`NextState<GameState>`] to control the game flow.
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
