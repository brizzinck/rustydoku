use bevy::prelude::*;

#[derive(Default, States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum StatePlaceholderAnimation {
    #[default]
    Idle,
    BouncingInit,
    BouncingDefault,
    BouncingPeak,
}

impl StatePlaceholderAnimation {
    pub fn when_idle(state: Res<State<StatePlaceholderAnimation>>) -> bool {
        StatePlaceholderAnimation::Idle == *state.get()
    }

    pub fn when_bouncing_init(state: Res<State<StatePlaceholderAnimation>>) -> bool {
        StatePlaceholderAnimation::BouncingInit == *state.get()
    }

    pub fn when_bouncing_default(state: Res<State<StatePlaceholderAnimation>>) -> bool {
        StatePlaceholderAnimation::BouncingDefault == *state.get()
    }

    pub fn when_bouncing_peak(state: Res<State<StatePlaceholderAnimation>>) -> bool {
        StatePlaceholderAnimation::BouncingPeak == *state.get()
    }
}
