use crate::{
    components::figure::square::SquareComponent, resource::square::SquaresToDespawnResource,
};
use bevy::prelude::*;

impl SquareComponent {
    pub(crate) fn call_despawn(
        mut commands: Commands,
        mut squares_to_despawn: ResMut<SquaresToDespawnResource>,
        mut squares: Query<&mut Sprite, With<SquareComponent>>,
        time: Res<Time>,
    ) {
        squares_to_despawn.squares.retain(|entity| {
            if let Ok(mut sprite) = squares.get_mut(*entity) {
                if SquareComponent::fading_out(&mut sprite.color, time.delta_secs()) {
                    commands.entity(*entity).despawn();
                    false
                } else {
                    true
                }
            } else {
                false
            }
        });
    }
}
