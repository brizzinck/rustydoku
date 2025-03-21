use bevy::prelude::*;

/// Name used in the UI hierarchy for the buttons panel.
pub const BUTTONS_PANEL_NAME_HIERARCHY: &str = "ButtonsPanel";

/// Relative positioning values for the buttons panel left.
pub const BUTTONS_PANEL_LEFT: Val = Val::Auto;

/// Relative positioning values for the buttons panel right.
pub const BUTTONS_PANEL_RIGHT: Val = Val::Auto;

/// Relative positioning values for the buttons panel bottom.
pub const BUTTONS_PANEL_BOTTOM: Val = Val::Auto;

/// Position type for the buttons panel.
pub const BUTTONS_PANEL_POSITION: PositionType = PositionType::Relative;

/// Margin for the buttons panel in the header.
pub const BUTTONS_PANEL_TOP: Val = Val::Auto;

/// Margin for the buttons panel in the header.
pub const BUTTONS_PANEL_MARGIN_HEADER: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Vw(5.),
    top: Val::Auto,
    bottom: Val::Auto,
};

/// Margin for the buttons panel in the game over panel.
pub const BUTTONS_PANEL_MARGIN_GAME_OVER: UiRect = UiRect {
    left: Val::Auto,
    right: Val::Auto,
    top: Val::Auto,
    bottom: Val::Auto,
};

/// Default width of the buttons panel.
pub const BUTTONS_PANEL_WIDTH: Val = Val::Auto;

/// Default height of the buttons panel.
pub const BUTTONS_PANEL_HEIGHT: Val = Val::Auto;

/// Maximum width for the buttons panel in the header.
pub const BUTTONS_PANEL_MAX_WIDTH_HEADER: Val = Val::Vw(25.);

/// Maximum height for the buttons panel in the header.
pub const BUTTONS_PANEL_MAX_HEIGHT_HEADER: Val = Val::Vh(8.);

/// Maximum width for the buttons panel in the game over panel.
pub const BUTTONS_PANEL_MAX_WIDTH_GAME_OVER: Val = Val::Auto;

/// Maximum height for the buttons panel in the game over panel.
pub const BUTTONS_PANEL_MAX_HEIGHT_GAME_OVER: Val = Val::Percent(20.);

/// Justification setting for aligning the buttons panel.
pub const BUTTONS_PANEL_JUSTIFY: JustifySelf = JustifySelf::End;

/// Horizontal space between buttons in the panel.
pub const BUTTONS_PANEL_COLUMN_GAP: Val = Val::Px(10.);
