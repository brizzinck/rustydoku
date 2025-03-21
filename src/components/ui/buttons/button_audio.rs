use crate::constants::ui::assets::*;
use crate::constants::ui::buttons::button_audio::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonAudio;

#[derive(Component)]
pub struct AudioImage;

impl ButtonAudio {
    pub(crate) fn spawn(commands: &mut ChildBuilder, assets: &AssetServer) {
        commands
            .spawn(ButtonAudio::create_button(
                ButtonAudio::create_node_game_over(),
                assets,
            ))
            .with_child(ButtonAudio::create_image(assets));
    }

    fn create_node_game_over() -> Node {
        Node {
            max_width: GAME_OVER_AUDIO_BUTTON_WIDTH,
            max_height: GAME_OVER_AUDIO_BUTTON_HEIGHT,
            justify_content: GAME_OVER_AUDIO_BUTTON_JUSTIFY,
            ..default()
        }
    }

    fn create_button(node: Node, assets: &AssetServer) -> impl Bundle {
        (
            node,
            Button,
            ButtonAudio,
            ImageNode {
                image: assets.load(BUTTON_BACKGROUND_PATH),
                ..default()
            },
        )
    }

    fn create_image(assets: &AssetServer) -> impl Bundle {
        (
            ImageNode {
                image: assets.load(AUDIO_ON_BUTTON_IMAGE_PATH),
                ..default()
            },
            AudioImage,
        )
    }
}
