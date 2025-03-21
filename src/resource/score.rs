use bevy::prelude::*;

/// A resource that tracks the player's score during gameplay.
///
/// Stores both the current score and the maximum score reached.
/// This is useful for displaying scoreboards, handling high-score logic, or triggering rewards.
#[derive(Resource, Default)]
pub struct ScoreResource {
    /// The current score value.
    current_value: i32,

    /// The maximum score value recorded so far.
    max_value: i32,
}

impl ScoreResource {
    /// Resets the current score to `0`.
    ///
    /// This is typically called when starting a new game or restarting.
    ///
    /// # Parameters
    /// - `score`: A mutable reference to the [`ScoreResource`] from the ECS world.
    pub(crate) fn reset_score(mut score: ResMut<ScoreResource>) {
        score.current_value = 0;
    }

    /// Adds the given value to the current score.
    ///
    /// # Parameters
    /// - `value`: The amount to add to the current score. Can be negative if needed.
    pub(crate) fn add_score(&mut self, value: i32) {
        self.current_value += value;
    }

    /// Updates the maximum score if the current score is greater than the previous maximum.
    ///
    /// This can be used at the end of a round to record high scores.
    ///
    /// # Parameters
    /// - `score`: A mutable reference to the [`ScoreResource`] from the ECS world.
    pub(crate) fn update_max_score(mut score: ResMut<ScoreResource>) {
        score.max_value = score.current_value.max(score.max_value);
    }

    /// Returns the current score value.
    ///
    /// # Returns
    /// - `i32`: The player's current score.
    pub(crate) fn get_current_score(&self) -> i32 {
        self.current_value
    }

    /// Returns the maximum score value recorded so far.
    ///
    /// # Returns
    /// - `i32`: The highest score the player has achieved.
    pub(crate) fn get_max_score(&self) -> i32 {
        self.max_value
    }
}
