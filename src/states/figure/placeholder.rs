use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlaceholderAnimationState {
    #[default]
    Idle,
    BouncingInit,
    BouncingDefault,
    BouncingPeak,
}

impl PlaceholderAnimationState {
    pub fn when_idle(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::Idle == *state.get()
    }

    pub fn when_bouncing_init(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::BouncingInit == *state.get()
    }

    pub fn when_bouncing_default(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::BouncingDefault == *state.get()
    }

    pub fn when_bouncing_peak(state: Res<State<PlaceholderAnimationState>>) -> bool {
        PlaceholderAnimationState::BouncingPeak == *state.get()
    }
}
