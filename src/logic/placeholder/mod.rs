use bevy::prelude::*;

use crate::{
    components::figures::Placeholder,
    constants::figure::assets::{
        FIGURE_PLACEHOLDER_DEFAULT_IMAGE_PATH, FIGURE_PLACEHOLDER_RED_IMAGE_PATH,
    },
    events::figure::{FigureCanPlaced, FigureCantPlaced},
};

impl Placeholder {
    pub(crate) fn change_image_by_placed(
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
