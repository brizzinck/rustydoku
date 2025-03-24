use crate::constants::ui::assets::*;
use crate::constants::ui::buttons::{button_restart::*, BACKGROUND_BUTTON_COLOR};
use crate::states::ui::button_restart::RestartButtonType;
use bevy::prelude::*;

/// A UI component representing a restart button.
///
/// Used in two contexts:
/// - Header bar restart button
/// - Game Over panel restart button
///
/// The button visually represents a restart action and carries a restart type
/// used for identifying which logic to execute when clicked.
#[derive(Component)]
pub struct ButtonRestart {
    pub(crate) restart_type: RestartButtonType,
}

impl ButtonRestart {
    /// Creates a new `ButtonRestart` component with a specified type.
    ///
    /// # Parameters
    /// - `_type`: A variant of `RestartButtonType` to identify button context (e.g., header or game over).
    ///
    /// # Returns
    /// A new instance of `ButtonRestart`.
    pub(crate) fn new(_type: RestartButtonType) -> Self {
        Self {
            restart_type: _type,
        }
    }

    /// Spawns the restart button inside the **header panel**.
    ///
    /// # Parameters
    /// - `commands`: The UI builder used to add child nodes.
    /// - `assets`: Bevy's asset server for loading textures.
    pub(crate) fn spawn_in_header(commands: &mut ChildBuilder, assets: &AssetServer) {
        commands
            .spawn(ButtonRestart::create_button(
                ButtonRestart::create_node(),
                assets,
                RestartButtonType::Default,
            ))
            .with_child(ButtonRestart::create_image(assets));
    }

    /// Spawns the restart button inside the **game over panel**.
    ///
    /// # Parameters
    /// - `commands`: The UI builder used to add child nodes.
    /// - `assets`: Bevy's asset server for loading textures.
    pub(crate) fn spawn_in_game_over(commands: &mut ChildBuilder, assets: &AssetServer) {
        commands
            .spawn(ButtonRestart::create_button(
                ButtonRestart::create_node(),
                assets,
                RestartButtonType::GameOver,
            ))
            .with_child(ButtonRestart::create_image(assets));
    }

    /// Internal helper to construct the visual node style for the button.
    ///
    /// # Node Parameters configured
    /// - `width`: Maximum width of the button.
    /// - `height`: Maximum height of the button.
    /// - `margin`: Margin around the button.
    /// - `justify`: Justify content alignment.
    ///
    /// # Returns
    /// A configured `Node` object.
    fn create_node() -> Node {
        Node {
            width: GAME_OVER_RESTART_BUTTON_WIDTH,
            height: GAME_OVER_RESTART_BUTTON_HEIGHT,
            max_width: GAME_OVER_RESTART_BUTTON_MAX_WIDTH,
            max_height: GAME_OVER_RESTART_BUTTON_MAX_HEIGHT,
            justify_content: GAME_OVER_RESTART_BUTTON_JUSTIFY,
            ..default()
        }
    }

    /// Internal helper to bundle together the components of a restart button.
    ///
    /// # Parameters
    /// - `node`: The visual style node.
    /// - `assets`: The asset server for loading button images.
    /// - `_type`: The restart type to embed in the component.
    ///
    /// # Returns
    /// A `Bundle` of UI and logical components.
    fn create_button(node: Node, assets: &AssetServer, _type: RestartButtonType) -> impl Bundle {
        (
            node,
            Button,
            ImageNode {
                image: assets.load(BUTTON_BACKGROUND_PATH),
                color: BACKGROUND_BUTTON_COLOR,
                ..default()
            },
            ButtonRestart::new(_type),
        )
    }

    /// Internal helper to generate the restart icon image.
    ///
    /// # Parameters
    /// - `assets`: The asset server for loading image resources.
    ///
    /// # Returns
    /// An `ImageNode` with the restart icon.
    fn create_image(assets: &AssetServer) -> ImageNode {
        ImageNode {
            image: assets.load(RESTART_BUTTON_IMAGE_PATH),
            ..default()
        }
    }
}
