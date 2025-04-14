/// Represents the type of restart button used in the UI.
///
/// This enum is used to distinguish between different restart buttons based on their context,
/// allowing for conditional logic depending on which button was pressed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RestartButtonType {
    /// Default restart button, now in only for header
    Default,
    /// Restart button for game over panel, have more logic, now only in gave over panel
    GameOver,
}
