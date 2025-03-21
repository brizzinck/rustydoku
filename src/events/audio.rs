use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ChangeVolumeEvent {
    pub(crate) music_volume: f64,
    pub(crate) sound_volume: f64,
}
