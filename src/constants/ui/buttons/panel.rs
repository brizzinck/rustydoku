use bevy::prelude::*;

pub const BUTTONS_PANEL_NAME_HIERARCHY: &str = "ButtonsPanel";

pub const BUTTONS_PANEL_LEFT: Val = Val::Auto;
pub const BUTTONS_PANEL_RIGHT: Val = Val::Auto;
pub const BUTTONS_PANEL_BOTTOM_HEADER: Val = Val::Auto;
pub const BUTTONS_PANEL_BOTTOM_GAME_OVER: Val = Val::Px(5.);
pub const BUTTONS_PANEL_POSITION: PositionType = PositionType::Relative;
pub const BUTTONS_PANEL_TOP: Val = Val::Auto;

pub const BUTTONS_PANEL_MARGIN_HEADER: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Vw(5.),
    top: Val::Auto,
    bottom: Val::Auto,
};

pub const BUTTONS_PANEL_MARGIN_GAME_OVER: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,
};

pub const BUTTONS_PANEL_WIDTH: Val = Val::Auto;
pub const BUTTONS_PANEL_HEIGHT: Val = Val::Auto;

pub const BUTTONS_PANEL_MAX_WIDTH: Val = Val::Vw(25.);
pub const BUTTONS_PANEL_MAX_HEIGHT: Val = Val::Vh(8.);

pub const BUTTONS_PANEL_JUSTIFY: JustifySelf = JustifySelf::End;

pub const BUTTONS_PANEL_COLUMN_GAP: Val = Val::Px(10.);
