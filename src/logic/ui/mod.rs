use crate::constants::{
    ui::DynamicText,
    world::window::{WINDOW_HEIGHT_SCALED_FACTOR, WINDOW_WIDTH_SCALED_FACTOR},
};
use bevy::prelude::*;

pub mod audio;
pub mod buttons;
pub mod game_over_panel;
pub mod header;

impl DynamicText {
    /// Dynamically adjusts the font sizes of text elements to match the current window resolution.
    ///
    /// # Type Parameters
    ///
    /// - `TextFont`: A mutable struct or component that has a `font_size: f32` field.
    ///
    /// # Arguments
    ///
    /// * `windows` - A query containing the primary window (must return only one).
    /// * `query` - A query that returns pairs of (`TextFont`, [`DynamicText`]) components.
    ///
    /// The method computes a scale factor based on the ratio between the actual window size
    /// and some predefined virtual size constants. It then scales the base font size using this factor,
    /// but ensures the result does not exceed the specified `max_font_size`.
    pub fn sized(
        windows: Query<&bevy::window::Window>,
        mut query: Query<(&mut TextFont, &DynamicText)>,
    ) {
        if let Ok(window) = windows.get_single() {
            for (mut font, text) in &mut query {
                let scale_x = window.width() / (WINDOW_WIDTH_SCALED_FACTOR * text.get_factor());
                let scale_y = window.height() / (WINDOW_HEIGHT_SCALED_FACTOR * text.get_factor());
                let scale = scale_x.min(scale_y);
                font.font_size = text.get_max_font_size().min(text.get_font_size() * scale);
            }
        }
    }
}
