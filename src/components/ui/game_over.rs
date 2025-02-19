use super::button_restart::spawn_restart_button_game_over;
use crate::{constants::ui::game_over_panel::*, resource::score::Score};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverPanel {
    pub timer: Timer,
    pub speed: f32,
}

impl GameOverPanel {
    pub fn spawn_game_over_panel(
        mut commands: Commands,
        assets: Res<AssetServer>,
        score: Res<Score>,
    ) {
        commands.spawn(create_panel()).with_children(|parent| {
            parent
                .spawn(create_background(&assets))
                .with_children(|panel| {
                    panel.spawn(create_header_title(&assets));

                    panel.spawn(create_max_score(score.max_value, &assets));

                    panel.spawn(create_current_score(score.current_value, &assets));

                    spawn_restart_button_game_over(panel, &assets);
                });
        });
    }
}

fn create_panel() -> (Node, GameOverPanel) {
    (
        Node {
            left: GAME_OVER_PANEL_LEFT,
            right: GAME_OVER_PANEL_RIGHT,
            bottom: GAME_OVER_PANEL_BOTTOM,
            position_type: GAME_OVER_PANEL_POSITION_TYPE,
            top: GAME_OVER_PANEL_TOP_DEFAULT,
            width: GAME_OVER_PANEL_WIDTH,
            height: GAME_OVER_PANEL_HEIGHT,
            margin: GAME_OVER_PANEL_MARGIN,
            max_width: GAME_OVER_PANEL_MAX_WIDTH,
            max_height: GAME_OVER_PANEL_MAX_HEIGHT,
            justify_self: GAME_OVER_PANEL_JUSTIFY_SELF,
            ..default()
        },
        GameOverPanel {
            timer: Timer::from_seconds(GAME_OVER_PANEL_TIMER_ANIMATION, TimerMode::Once),
            speed: GAME_OVER_PANEL_SPEED_ANIMATION,
        },
    )
}

fn create_background(assets: &Res<AssetServer>) -> (Node, ImageNode) {
    (
        Node {
            width: GAME_OVER_BACKGROUND_WIDTH,
            height: GAME_OVER_BACKGROUND_HEIGHT,
            flex_direction: GAME_OVER_BACKGROUND_FLEX_DIRECTION,
            justify_content: GAME_OVER_BACKGROUND_JUSTIFY_CONTENT,
            align_items: GAME_OVER_BACKGROUND_ALIGN_ITEMS,
            margin: GAME_OVER_BACKGROUND_MARGIN,
            row_gap: GAME_OVER_BACKGROUND_ROW_GAP,
            ..default()
        },
        ImageNode {
            image: assets.load(GAME_OVER_BACKGROUND_IMAGE_PATH),
            color: GAME_OVER_BACKGROUND_COLOR.into(),
            ..default()
        },
    )
}

fn create_header_title(assets: &Res<AssetServer>) -> (Text, TextFont, TextColor) {
    (
        Text::new(HEADER_TITLE_CONTENT),
        TextFont {
            font_size: HEADER_TITLE_FONT_SIZE,
            font: assets.load(HEADER_TITLE_FONT_PATH),
            ..default()
        },
        TextColor(HEADER_TITLE_COLOR),
    )
}

fn create_current_score(score: i32, assets: &Res<AssetServer>) -> (Text, TextFont, TextColor) {
    (
        Text::new(format!("{SCORE_TEXT_CONTENT}: {score}")),
        TextFont {
            font_size: SCORE_TEXT_FONT_SIZE,
            font: assets.load(SCORE_TEXT_FONT_PATH),
            ..default()
        },
        TextColor(SCORE_TEXT_COLOR),
    )
}

fn create_max_score(max_score: i32, assets: &Res<AssetServer>) -> (Text, TextFont, TextColor) {
    (
        Text::new(format!("{MAX_SCORE_TEXT_CONTENT}: {max_score}")),
        TextFont {
            font_size: MAX_SCORE_TEXT_FONT_SIZE,
            font: assets.load(MAX_SCORE_TEXT_FONT_PATH),
            ..default()
        },
        TextColor(MAX_SCORE_TEXT_COLOR),
    )
}
