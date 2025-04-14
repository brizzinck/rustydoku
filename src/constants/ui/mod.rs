use bevy::ecs::component::Component;

pub mod assets;
pub mod audio;
pub mod buttons;
pub mod game_over_panel;
pub mod header;

/// A UI helper component used to scale text dynamically depending on the window size.
///
/// This component stores font scaling parameters for a UI text element.
/// It is useful when you want text to remain readable and proportional
/// across different window resolutions.
///
/// The final font size is calculated as:
/// `min(max_font_size, font_size * scale)`
///
/// where `scale` is based on the current window dimensions and the provided scaling `factor`.
#[derive(Component)]
pub struct DynamicText {
    /// The base font size before scaling.
    font_size: f32,
    /// A scaling factor that determines how responsive the text is to window size.
    factor: f32,
    /// A hard upper limit for how large the font can be, regardless of scaling.
    max_font_size: f32,
}

impl DynamicText {
    /// Creates a new [`DynamicText`] component instance.
    ///
    /// # Arguments
    ///
    /// * `font_size` - The base size of the font.
    /// * `factor` - How aggressively the font should scale with window size.
    /// * `max_font_size` - The maximum size the font is allowed to scale up to.
    pub fn new(font_size: f32, factor: f32, max_font_size: f32) -> Self {
        Self {
            font_size,
            factor,
            max_font_size,
        }
    }

    /// Returns the base font size.
    pub fn get_font_size(&self) -> f32 {
        self.font_size
    }

    /// Returns the scaling factor.
    pub fn get_factor(&self) -> f32 {
        self.factor
    }

    /// Returns the maximum font size.
    pub fn get_max_font_size(&self) -> f32 {
        self.max_font_size
    }
}

impl Default for DynamicText {
    /// Creates a default [`DynamicText`] with:
    /// - `font_size` = 32.0
    /// - `factor` = 1.0
    /// - `max_font_size` = 42.0
    fn default() -> Self {
        Self {
            font_size: 32.,
            factor: 1.,
            max_font_size: 42.,
        }
    }
}
