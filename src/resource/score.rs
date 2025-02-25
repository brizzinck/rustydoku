use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    current_value: i32,
    max_value: i32,
}

impl Score {
    pub(crate) fn reset_score(mut score: ResMut<Score>) {
        info!("Resetting score");
        score.current_value = 0;
    }

    pub(crate) fn add_score(&mut self, value: i32) {
        self.current_value += value;
    }

    pub(crate) fn update_max_score(mut score: ResMut<Score>) {
        score.max_value = score.current_value.max(score.max_value);
    }

    pub(crate) fn get_current_score(&self) -> i32 {
        self.current_value
    }

    pub(crate) fn get_max_score(&self) -> i32 {
        self.max_value
    }
}
