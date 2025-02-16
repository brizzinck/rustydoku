use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub current_value: i32,
    pub max_value: i32,
}

pub fn restart_score(mut score: ResMut<Score>) {
    score.current_value = 0;
}

pub fn update_max_score(mut score: ResMut<Score>) {
    score.max_value = score.current_value.max(score.max_value);
}
