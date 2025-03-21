use crate::constants::ui::assets::*;
use crate::constants::ui::buttons::button_audio::*;
use bevy::prelude::*;

/// Component for the audio toggle button.
#[derive(Component)]
pub struct ButtonAudio;

/// Marker component used to target and update the image of the audio button.
#[derive(Component)]
pub struct AudioImage;

impl ButtonAudio {
    /// Spawns the audio toggle button in the game over panel.
    ///
    /// The button includes a background and an icon, which can be updated based on mute state.
    ///
    /// # Parameters
    /// - `commands`: ChildBuilder used to spawn UI elements within a parent.
    /// - `assets`: Reference to the asset server for loading textures.
    pub(crate) fn spawn(commands: &mut ChildBuilder, assets: &AssetServer) {
        commands
            .spawn(ButtonAudio::create_button(
                ButtonAudio::create_node_game_over(),
                assets,
            ))
            .with_child(ButtonAudio::create_image(assets));
    }

    /// Creates the node layout for the audio button in the game over panel.
    ///
    /// # Returns
    /// A `Node` configured for styling the audio button container.
    fn create_node_game_over() -> Node {
        Node {
            max_width: GAME_OVER_AUDIO_BUTTON_WIDTH,
            max_height: GAME_OVER_AUDIO_BUTTON_HEIGHT,
            justify_content: GAME_OVER_AUDIO_BUTTON_JUSTIFY,
            ..default()
        }
    }

    /// Creates the main button bundle with background styling and interaction capability.
    ///
    /// # Parameters
    /// - `node`: The base `Node` to apply layout styling.
    /// - `assets`: Reference to the asset server for loading the background image.
    ///
    /// # Returns
    /// A bundle including the button logic and visual background.
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

    /// Creates the icon image bundle to be used inside the button.
    ///
    /// This image will reflect the current mute state (on/off).
    ///
    /// # Parameters
    /// - `assets`: Reference to the asset server for loading the icon.
    ///
    /// # Returns
    /// A bundle with the initial audio icon and `AudioImage` marker.
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
