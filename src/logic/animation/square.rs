use crate::{components::figure::square::SquareComponent, constants::square::*};
use bevy::prelude::*;

impl SquareComponent {
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

    #[test]
    fn fading_out_works() {
        let mut color = Color::srgb(1., 1., 1.);
        let delta = 0.1;

        let result = super::SquareComponent::fading_out(&mut color, delta);

        let expected_result = false;
        assert_eq!(result, expected_result);

        let expected_color = Color::srgba(1., 1., 1., 0.8);
        assert_eq!(color, expected_color);
    }
}
