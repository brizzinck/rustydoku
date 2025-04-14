use crate::{components::figure::square::SquareComponent, constants::square::*};
use bevy::prelude::*;

impl SquareComponent {
    /// Gradually fades out the square’s color alpha channel.
    ///
    /// This function linearly interpolates the square’s current alpha toward the target alpha value
    /// defined by [`SQUARE_ALPHA_TARGET_COMBO`]. The speed of interpolation is scaled by the
    /// frame `delta` time and a configured fade-out speed multiplier.
    ///
    /// This is typically used during combo clearing animations, where squares visually fade
    /// out before being despawned.
    ///
    /// # Parameters
    /// - `color`: Mutable reference to the color of the square (includes alpha).
    /// - `delta`: Time delta between frames (in seconds), used to smooth the animation.
    ///
    /// # Returns
    /// - `true` if the square is fully transparent (ready to despawn).
    /// - `false` if fading is still in progress.
    pub(crate) fn fading_out(color: &mut Color, delta: f32) -> bool {
        let srgb = color.to_srgba().with_alpha(color.alpha().lerp(
            SQUARE_ALPHA_TARGET_COMBO,
            delta
                * SQUARE_ANIMATION_ALPHA_SPEED
                * (SQUARE_FADE_OUT_SPEED_INCREMENT_PER_FRAME + color.alpha()),
        ));

        if color.alpha() <= 0.01 {
            *color = srgb.with_alpha(0.).into();
            true
        } else {
            *color = srgb.into();
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    /// Tests the fading out logic of a square component.
    ///
    /// This verifies that the alpha value decreases over time and that
    /// `fading_out` returns the correct boolean when fading is complete or in progress.
    #[test]
    fn fading_out_works() {
        let mut color = Color::srgb(1., 1., 1.);
        let delta = 0.1;

        let result = super::SquareComponent::fading_out(&mut color, delta);

        let expected_result = false;
        assert_eq!(result, expected_result);

        let expected_color = Color::srgba(1., 1., 1., 0.5);
        assert_eq!(color, expected_color);
    }
}
