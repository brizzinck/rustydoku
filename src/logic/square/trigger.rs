use crate::{components::square::Square, resource::square::SquaresToDespawn};
use bevy::prelude::*;

impl Square {
    pub(crate) fn call_despawn(
        mut commands: Commands,
        mut squares_to_despawn: ResMut<SquaresToDespawn>,
        mut squares: Query<&mut Sprite, With<Square>>,
        time: Res<Time>,
    ) {
        squares_to_despawn.squares.retain(|entity| {
            if let Ok(mut sprite) = squares.get_mut(*entity) {
                if Square::fading_out(&mut sprite.color, time.delta_secs()) {
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
