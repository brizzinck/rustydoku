use crate::constants::ui::assets::*;
use crate::constants::ui::restart_button::*;
use crate::states::ui::restart_button::RestartButtonType;
use bevy::prelude::*;

#[derive(Component)]
pub struct RestartButton {
    pub(crate) restart_type: RestartButtonType,
}

impl RestartButton {
    pub(crate) fn new(_type: RestartButtonType) -> Self {
        Self {
            restart_type: _type,
        }
    }

    pub(crate) fn spawn_in_header(commands: &mut ChildBuilder, assets: &AssetServer) {
        commands
            .spawn(RestartButton::create_button(
                RestartButton::create_node(),
                assets,
                RestartButtonType::Default,
            ))
            .with_child(RestartButton::create_image(assets));
    }

    pub(crate) fn spawn_in_game_over(commands: &mut ChildBuilder, assets: &AssetServer) {
        commands
            .spawn(RestartButton::create_button(
                RestartButton::create_node(),
                assets,
                RestartButtonType::GameOver,
            ))
            .with_child(RestartButton::create_image(assets));
    }

    fn create_node() -> Node {
        Node {
            max_width: GAME_OVER_RESTART_BUTTON_WIDTH,
            max_height: GAME_OVER_RESTART_BUTTON_HEIGHT,
            justify_content: GAME_OVER_RESTART_BUTTON_JUSTIFY,
            ..default()
        }
    }

    fn create_button(node: Node, assets: &AssetServer, _type: RestartButtonType) -> impl Bundle {
        (
            node,
            Button,
            ImageNode {
                image: assets.load(BUTTON_BACKGROUND_PATH),
                ..default()
            },
            RestartButton::new(_type),
        )
    }

    fn create_image(assets: &AssetServer) -> ImageNode {
        ImageNode {
            image: assets.load(RESTART_BUTTON_IMAGE_PATH),
            ..default()
        }
    }
}
