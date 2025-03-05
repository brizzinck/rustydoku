pub mod placeholder;

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub enum StateFigureAnimation {
    #[default]
    Idle,
    SpawnUpScaling,
    BackLerping,
    DragUpScaling,
}

impl StateFigureAnimation {
    pub fn is_default(&self) -> bool {
        matches!(self, StateFigureAnimation::Idle)
    }

    pub fn is_spawn_upscaling(&self) -> bool {
        matches!(self, StateFigureAnimation::SpawnUpScaling)
    }
}
