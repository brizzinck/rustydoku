use bevy::prelude::*;

#[derive(Component)]
pub struct RustyCamera2D;

impl RustyCamera2D {
    pub(crate) fn spawn(mut commands: Commands) {
        commands.spawn((Camera2d, RustyCamera2D));
    }
}
