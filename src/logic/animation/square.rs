use crate::{components::figure::square::Square, constants::square::*};
use bevy::prelude::*;

impl Square {
    pub(crate) fn fading_out(color: &mut Color, delta: f32) -> bool {
        let srgb = color.to_srgba().with_alpha(color.alpha().lerp(
            SQUARE_ALPHA_TARGET_WHEN_COMBO,
            delta
                * SQUARE_ALPHA_SPEED_ANIMATION
                * (SQUARE_FADE_OUT_SPEED_INCREMENT_FACTOR_PER_FRAME + color.alpha()),
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
