use bevy::prelude::*;

use crate::{
    components::world::placeholder::PlaceholderComponent,
    events::figure::{FigureCanPlacedEvent, FigureCantPlacedEvent},
    resource::figure_spawner::FigureSpawnerResource,
};

impl PlaceholderComponent {
    pub(crate) fn update_image(
        mut placeholders: Query<&mut Sprite, With<PlaceholderComponent>>,
        mut cant_place: EventReader<FigureCantPlacedEvent>,
        mut can_place: EventReader<FigureCanPlacedEvent>,
        resource: Res<FigureSpawnerResource>,
    ) {
        for FigureCantPlacedEvent(placeholder) in cant_place.read() {
            if let Ok(mut sprite) = placeholders.get_mut(*placeholder) {
                sprite.image = resource.get_placeholder_image_deactive();
                trace!("Placeholder {} is deactive", placeholder);
            }
        }

        for FigureCanPlacedEvent(placeholder) in can_place.read() {
            if let Ok(mut sprite) = placeholders.get_mut(*placeholder) {
                sprite.image = resource.get_placeholder_image();
                trace!("Placeholder {} is active", placeholder);
            }
        }
    }

    pub(crate) fn reset_image(
        mut placeholders: Query<&mut Sprite, With<PlaceholderComponent>>,
        resource: Res<FigureSpawnerResource>,
    ) {
        trace!("Reset placeholders image");

        for mut sprite in placeholders.iter_mut() {
            sprite.image = resource.get_placeholder_image();
        }
    }
}
