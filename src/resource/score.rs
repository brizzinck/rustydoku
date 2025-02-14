use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub value: i32,
}

pub fn restart_score(mut score: ResMut<Score>) {
    score.value = 0;
}
