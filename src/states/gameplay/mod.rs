use bevy::prelude::*;

/// Represents the main game state, controlling the overall flow and phases of the game.
///
/// This enum defines various stages in the gameplay loop, such as initialization, world generation,
/// idle waiting, user interaction (dragging/placing), and game status checks (combo or game over).
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// The initial state, used for setting up required resources at game start.
    #[default]
    InitResources,

    /// The second state, responsible for generating the game world (map, tiles, etc.).
    GenerateWorld,

    /// The default gameplay state when the player is not interacting with any figure.
    Idle,

    /// State when the player is actively dragging a figure.
    ///
    /// Contains the [`Entity`] of the dragged figure.
    Dragging(Entity),

    /// State when the player releases the figure and attempts to place it on the board.
    ///
    /// Contains the [`Entity`] of the figure being placed.
    Placing(Entity),

    /// State after a figure has been successfully placed on the board.
    ///
    /// Contains the [`Entity`] of the placed figure.
    Placed(Entity),

    /// State triggered after placement to check for combos (e.g. clearing lines or scoring bonuses).
    CheckCombo,

    /// State triggered after checking for combos to evaluate if the game is over.
    CheckGameOver,

    /// The game has ended (no more moves or conditions met).
    GameOver,

    /// The game is being restarted via the header restart button.
    DefaultRestart,

    /// The game is being restarted from the game over screen.
    GameOverRestart,
}

impl GameState {
    /// Returns `true` if the current state is [`GameState::Idle`].
    ///
    /// # Parameters
    /// - `state`: A Bevy [`State<GameState>`] resource.
    ///
    /// # Returns
    /// - `true` if the state is `Idle`.
    pub fn when_idle(state: Res<State<GameState>>) -> bool {
        GameState::Idle == *state.get()
    }

    /// Returns `true` if the current state is [`GameState::Dragging`].
    ///
    /// # Parameters
    /// - `state`: A Bevy [`State<GameState>`] resource.
    ///
    /// # Returns
    /// - `true` if the state is `Dragging(_)`.
    pub fn when_draggin(state: Res<State<GameState>>) -> bool {
        matches!(state.get(), GameState::Dragging(_))
    }

    /// Returns `true` if the current state is [`GameState::Placing`].
    ///
    /// # Parameters
    /// - `state`: A Bevy [`State<GameState>`] resource.
    ///
    /// # Returns
    /// - `true` if the state is `Placing(_)`.
    pub fn when_placing(state: Res<State<GameState>>) -> bool {
        matches!(state.get(), GameState::Placing(_))
    }

    /// Returns `true` if the current state is [`GameState::Placed`].
    ///
    /// # Parameters
    /// - `state`: A Bevy [`State<GameState>`] resource.
    ///
    /// # Returns
    /// - `true` if the state is `Placed(_)`.
    pub fn when_placed(state: Res<State<GameState>>) -> bool {
        matches!(state.get(), GameState::Placed(_))
    }
}
