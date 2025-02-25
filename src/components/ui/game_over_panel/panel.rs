use super::{super::button_restart::RestartButton, score_text::*};
use crate::constants::ui::game_over_panel::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverPanel {
    pub timer: Timer,
    pub speed: f32,
}

impl GameOverPanel {
    pub fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
        commands
            .spawn(GameOverPanel::create_panel())
            .with_children(|parent| {
                parent
                    .spawn(GameOverPanel::create_background(&assets))
                    .with_children(|panel| {
                        panel.spawn(GameOverPanel::create_header_title(&assets));

                        panel.spawn(GameOverMaxScoreText::create_max_score(&assets));

                        panel.spawn(GameOverCurrentScoreText::create_current_score(&assets));

                        RestartButton::spawn_in_game_over(panel, &assets);
                    });
            });
    }

    fn create_panel() -> impl Bundle {
        (
            Name::new(GAME_OVER_PANEL_NAME_HIERARCHY),
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
                speed: GAME_OVER_PANEL_DEFAULT_SPEED_ANIMATION,
            },
        )
    }

    fn create_background(assets: &Res<AssetServer>) -> impl Bundle {
        (
            Name::new(GAME_OVER_BACKGROUND_NAME_HIERARCHY),
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
}
