use bevy::prelude::*;

/// Represents the current animation state of a figure placeholder.
///
/// Used to control the visual bouncing animation that occurs when a placeholder becomes active or interacts with a figure.
/// The animation goes through several stages: from idle, through bounce initialization and peak, then returning to default.
#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlaceholderAnimationState {
    /// The default state when no animation is active.
    #[default]
    Idle,
    /// The initial stage of the bounce animation (e.g., starting to lift).
    BouncingInit,
    /// The peak stage of the bounce animation (e.g., highest point).
    BouncingPeak,
    /// The returning stage of the bounce animation, going back to idle.
    BouncingDefault,
}

impl PlaceholderAnimationState {
    /// Returns `true` if the current placeholder state is [`PlaceholderAnimationState::Idle`].
    ///
    /// # Parameters
    /// - `state`: A [`Res<State<PlaceholderAnimationState>>`] representing the current state.
    ///
    /// # Returns
    /// - `true` if the state is `Idle`, otherwise `false`.
    pub fn when_idle(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::Idle == *state.get()
    }

    /// Returns `true` if the current placeholder state is [`PlaceholderAnimationState::BouncingInit`].
    ///
    /// Indicates the bounce animation has just started.
    pub fn when_bouncing_init(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::BouncingInit == *state.get()
    }

    /// Returns `true` if the current placeholder state is [`PlaceholderAnimationState::BouncingDefault`].
    ///
    /// Indicates the bounce is in the final stage, returning to idle.
    pub fn when_bouncing_default(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::BouncingDefault == *state.get()
    }

    /// Returns `true` if the current placeholder state is [`PlaceholderAnimationState::BouncingPeak`].
    ///
    /// Indicates the animation is at its maximum upward point.
    pub fn when_bouncing_peak(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::BouncingPeak == *state.get()
    }
}
