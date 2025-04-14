use crate::{
    components::figure::square::SquareComponent, resource::square::SquaresToDespawnResource,
};
use bevy::prelude::*;

impl SquareComponent {
    /// Fades out and despawns squares that are marked for removal.
    ///
    /// This function:
    /// - Iterates through all entities listed in [`SquaresToDespawnResource`].
    /// - Gradually reduces their alpha (transparency) using [`SquareComponent::fading_out`].
    /// - Despawns the square once it is fully transparent.
    ///
    /// Squares that are not fully faded out remain in the despawn list and continue to be processed in the next frame.
    ///
    /// # Parameters
    /// - `commands`: Bevy [`Commands`] used to despawn entities.
    /// - `squares_to_despawn`: Resource tracking which square entities should be faded out and removed.
    /// - `squares`: Query for mutable access to square sprites.
    /// - `time`: Time resource for frame delta (used to calculate fade speed).
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
