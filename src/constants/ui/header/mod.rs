use bevy::prelude::*;

pub mod assets;
pub mod score;

pub use assets::*;
pub use score::*;

pub const HEADER_NAME_HIERARCHY: &str = "Header UI";

pub const HEADER_LEFT: Val = Val::Percent(2.5);
pub const HEADER_TOP: Val = Val::Px(10.);
pub const HEADER_WIDTH: Val = Val::Vw(95.);
