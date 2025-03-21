use bevy::prelude::*;

use crate::{
    components::world::placeholder::Placeholder,
    events::figure::{FigureCanPlaced, FigureCantPlaced},
    resource::figure_spawner::FigureSpawner,
};

impl Placeholder {
    pub(crate) fn update_image(
        mut placeholders: Query<&mut Sprite, With<Placeholder>>,
        mut cant_place: EventReader<FigureCantPlaced>,
        mut can_place: EventReader<FigureCanPlaced>,
        resource: Res<FigureSpawner>,
    ) {
        for FigureCantPlaced(placeholder) in cant_place.read() {
            if let Ok(mut sprite) = placeholders.get_mut(*placeholder) {
                sprite.image = resource.get_placeholder_image_deactive();
                trace!("Placeholder {} is deactive", placeholder);
            }
        }

        for FigureCanPlaced(placeholder) in can_place.read() {
            if let Ok(mut sprite) = placeholders.get_mut(*placeholder) {
                sprite.image = resource.get_placeholder_image();
                trace!("Placeholder {} is active", placeholder);
            }
        }
    }

    pub(crate) fn reset_image(
        mut placeholders: Query<&mut Sprite, With<Placeholder>>,
        resource: Res<FigureSpawner>,
    ) {
        trace!("Reset placeholders image");

        for mut sprite in placeholders.iter_mut() {
            sprite.image = resource.get_placeholder_image();
        }
    }
}
