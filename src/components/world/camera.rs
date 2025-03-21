use bevy::prelude::*;

#[derive(Component)]
pub struct RustyCamera2DComponent;

impl RustyCamera2DComponent {
    pub(crate) fn spawn(mut commands: Commands) {
        commands.spawn((Camera2d, RustyCamera2DComponent));
    }
}
