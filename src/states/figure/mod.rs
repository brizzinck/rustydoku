pub mod placeholder;

/// Represents the animation state of a figure during its lifecycle.
///
/// This enum is used to control figure animations such as spawning, dragging, and
/// returning to the placeholder. Systems can use this to determine the correct animation
/// behavior for a figure at any given time.
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub enum FigureAnimationState {
    /// The default state. The figure is idle and positioned in its placeholder.
    #[default]
    Idle,
    /// The figure is spawning and scaling up (e.g., entering the game board).
    SpawnUpScaling,
    /// The figure is returning to its placeholder (e.g., after invalid placement).
    BackLerping,
    /// The figure is being dragged and scaled up to provide visual feedback.
    DragUpScaling,
}

impl FigureAnimationState {
    /// Returns `true` if the current state is [`FigureAnimationState::Idle`].
    ///
    /// This is used to check whether the figure is idle and ready for interaction.
    ///
    /// # Returns
    /// - `true` if the state is `Idle`, `false` otherwise.
    pub fn is_default(&self) -> bool {
        matches!(self, FigureAnimationState::Idle)
    }

    /// Returns `true` if the current state is [`FigureAnimationState::SpawnUpScaling`].
    ///
    /// This is used to detect when a figure is in the spawning animation state.
    ///
    /// # Returns
    /// - `true` if the state is `SpawnUpScaling`, `false` otherwise.
    pub fn is_spawn_upscaling(&self) -> bool {
        matches!(self, FigureAnimationState::SpawnUpScaling)
    }
}
