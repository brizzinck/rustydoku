use bevy::prelude::*;

pub mod assets;
pub mod score;

pub use assets::*;
pub use score::*;

/// Name of the header UI.
pub const HEADER_NAME_HIERARCHY: &str = "Header UI";

/// Relative positioning values for the header left.
pub const HEADER_LEFT: Val = Val::Percent(2.5);

/// Relative positioning values for the header right.
pub const HEADER_TOP: Val = Val::Px(10.);

/// Relative positioning values for the header bottom.
pub const HEADER_WIDTH: Val = Val::Vw(95.);
