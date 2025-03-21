pub mod placeholder;

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub enum FigureAnimationState {
    #[default]
    Idle,
    SpawnUpScaling,
    BackLerping,
    DragUpScaling,
}

impl FigureAnimationState {
    pub fn is_default(&self) -> bool {
        matches!(self, FigureAnimationState::Idle)
    }

    pub fn is_spawn_upscaling(&self) -> bool {
        matches!(self, FigureAnimationState::SpawnUpScaling)
    }
}
