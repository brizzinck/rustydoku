#[derive(PartialEq, Clone, Debug, Default)]
pub enum StatePlaceholderAnimation {
    #[default]
    Idle,
    BouncingInit,
    BouncingDefault,
    BouncingPeak,
}
