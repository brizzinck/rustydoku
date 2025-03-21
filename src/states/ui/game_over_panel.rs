use bevy::prelude::*;

/// Represents the animation state of the Game Over panel UI.
///
/// This state is used to control how and when the game over panel appears and disappears,
/// typically in response to a game loss or restart. It is used for animation timing.
#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameOverPanelState {
    /// The panel is in the process of appearing (e.g. animating in).
    Showing,

    /// The panel is fully visible on the screen.
    Showed,

    /// The panel is in the process of disappearing (e.g. animating out).
    Hidding,

    /// The panel is fully hidden. This is the default or idle state.
    #[default]
    Hidden,
}

impl GameOverPanelState {
    /// Returns `true` if the current state is [`GameOverPanelState::Showing`].
    ///
    /// Useful for running systems only during the panel's show animation.
    pub fn when_showing(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Showing == *state.get()
    }

    /// Returns `true` if the current state is [`GameOverPanelState::Showed`].
    ///
    /// Indicates the panel is fully visible and interactive.
    pub fn when_showed(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Showed == *state.get()
    }

    /// Returns `true` if the current state is [`GameOverPanelState::Hidding`].
    ///
    /// Indicates the panel is animating out (e.g. after a restart).
    pub fn when_hidding(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Hidding == *state.get()
    }

    /// Returns `true` if the current state is [`GameOverPanelState::Hidden`].
    ///
    /// Indicates the panel is not visible. This is the default or inactive state.
    pub fn when_hidded(state: Res<State<GameOverPanelState>>) -> bool {
        GameOverPanelState::Hidden == *state.get()
    }
}
