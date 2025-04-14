use bevy::ecs::event::Event;

/// Event triggered when audio volume settings are changed.
///
/// This event updates both background music and sound effect volumes throughout the application.
/// Volume values range from `0.0` (muted) to `1.0` (maximum volume).
#[derive(Event)]
pub struct ChangeVolumeEvent {
    /// The updated volume level for background music.
    pub(crate) music_volume: f64,
    /// The updated volume level for sound effects.
    pub(crate) sound_volume: f64,
}
