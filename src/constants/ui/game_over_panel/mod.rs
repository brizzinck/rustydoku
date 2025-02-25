use bevy::prelude::*;

pub mod assets;
pub mod background;
pub mod header_title;
pub mod score_text;

pub use assets::*;
pub use background::*;
pub use header_title::*;
pub use score_text::*;

pub const GAME_OVER_PANEL_LEFT: Val = Val::Auto;
pub const GAME_OVER_PANEL_RIGHT: Val = Val::Auto;
pub const GAME_OVER_PANEL_BOTTOM: Val = Val::Auto;
pub const GAME_OVER_PANEL_POSITION_TYPE: PositionType = PositionType::Absolute;
pub const GAME_OVER_PANEL_TOP_DEFAULT_FLOAT: f32 = 120.;
pub const GAME_OVER_PANEL_TOP_DEFAULT: Val = Val::Percent(GAME_OVER_PANEL_TOP_DEFAULT_FLOAT);
pub const GAME_OVER_PANEL_WIDTH: Val = Val::Percent(132.);
pub const GAME_OVER_PANEL_HEIGHT: Val = Val::Percent(26.);
pub const GAME_OVER_PANEL_MARGIN: UiRect = UiRect {
    left: Val::Percent(0.),
    right: Val::Percent(0.),
    top: Val::Px(0.),
    bottom: Val::Px(0.),
};
pub const GAME_OVER_PANEL_MAX_WIDTH: Val = Val::Px(500.);
pub const GAME_OVER_PANEL_MAX_HEIGHT: Val = Val::Px(255.);
pub const GAME_OVER_PANEL_JUSTIFY_SELF: JustifySelf = JustifySelf::Center;

pub const GAME_OVER_PANEL_TIMER_ANIMATION: f32 = 1.;
pub const GAME_OVER_PANEL_DEFAULT_SPEED_ANIMATION: f32 = 1.;
pub const GAME_OVER_PANEL_SHOW_MUL_SPEED_ANIMATION: f32 = 2.;
pub const GAME_OVER_PANEL_HIDE_MUL_SPEED_ANIMATION: f32 = 25.;
pub const GAME_OVER_PANEL_TOP_END_FLOAT: f32 = GAME_OVER_PANEL_TOP_DEFAULT_FLOAT / 2. - 10.;
