use bevy::prelude::*;

use crate::{
    components::world::placeholder::PlaceholderComponent,
    events::figure::{FigureCanPlacedEvent, FigureCantPlacedEvent},
    resource::figure_spawner::FigureSpawnerResource,
};

impl PlaceholderComponent {
    /// Updates the image of placeholders based on whether a figure can or cannot be placed on them.
    ///
    /// This method listens for [`FigureCantPlacedEvent`] and [`FigureCanPlacedEvent`] events and updates
    /// the placeholder's sprite accordingly:
    /// - If a figure **cannot** be placed, the placeholder is set to the *deactivated* image.
    /// - If a figure **can** be placed, the placeholder is set to the *default/active* image.
    ///
    /// This provides visual feedback to the player about placement validity.
    ///
    /// # Parameters
    /// - `placeholders`: A query to access mutable `Sprite` components of placeholders.
    /// - `cant_place`: Event reader for events where a figure cannot be placed.
    /// - `can_place`: Event reader for events where a figure can be placed.
    /// - `resource`: Access to the figure spawner resource, which contains image handles.
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

    /// Resets all placeholder images to the default (active) state.
    ///
    /// This is typically called when the game is reset or when figure placement is no longer being evaluated.
    ///
    /// # Parameters
    /// - `placeholders`: A query to access mutable `Sprite` components of placeholders.
    /// - `resource`: Access to the figure spawner resource for the default placeholder image.
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
