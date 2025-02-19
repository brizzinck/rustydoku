#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub enum StateFigureAnimation {
    #[default]
    Idle,
    BackLerping,
    UpScaling,
}

impl StateFigureAnimation {
    pub fn is_default(&self) -> bool {
        matches!(self, StateFigureAnimation::Idle)
    }
}
