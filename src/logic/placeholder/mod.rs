use bevy::prelude::*;

use crate::{
    components::world::placeholder::Placeholder,
    constants::placeholder::assets::*,
    events::figure::{FigureCanPlaced, FigureCantPlaced},
};

impl Placeholder {
    pub(crate) fn update_image(
        mut placeholders: Query<&mut Sprite, With<Placeholder>>,
        mut cant_place: EventReader<FigureCantPlaced>,
        mut can_place: EventReader<FigureCanPlaced>,
        assets: Res<AssetServer>,
    ) {
        for FigureCantPlaced(placeholder) in cant_place.read() {
            if let Ok(mut sprite) = placeholders.get_mut(*placeholder) {
                sprite.image = assets.load(FIGURE_PLACEHOLDER_RED_IMAGE_PATH);
            }
        }

        for FigureCanPlaced(placeholder) in can_place.read() {
            if let Ok(mut sprite) = placeholders.get_mut(*placeholder) {
                sprite.image = assets.load(FIGURE_PLACEHOLDER_DEFAULT_IMAGE_PATH);
            }
        }
    }

    pub(crate) fn reset_image(
        mut placeholders: Query<&mut Sprite, With<Placeholder>>,
        assets: Res<AssetServer>,
    ) {
        for mut sprite in placeholders.iter_mut() {
            sprite.image = assets.load(FIGURE_PLACEHOLDER_DEFAULT_IMAGE_PATH);
        }
    }
}
