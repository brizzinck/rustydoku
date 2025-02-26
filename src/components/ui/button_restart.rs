use crate::constants::ui::assets::*;
use crate::constants::ui::restart_button::*;
use crate::states::gameplay::StateGame;
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

    pub(crate) fn spawn_in_header(
        commands: &mut Commands,
        parent: Entity,
        assets: &Res<AssetServer>,
    ) {
        commands
            .spawn(RestartButton::create_button(
                RestartButton::create_node_header(),
                assets,
                RestartButtonType::Default,
            ))
            .with_child(RestartButton::create_image(assets))
            .set_parent(parent);
    }

    pub(crate) fn spawn_in_game_over(commands: &mut ChildBuilder, assets: &Res<AssetServer>) {
        commands
            .spawn(RestartButton::create_button(
                RestartButton::create_node_game_over(),
                assets,
                RestartButtonType::GameOver,
            ))
            .with_child(RestartButton::create_image(assets));
    }

    pub(crate) fn handle(
        mut interaction_query: Query<(&Interaction, &RestartButton), Changed<Interaction>>,
        mut state: ResMut<NextState<StateGame>>,
    ) {
        for (interaction, button) in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                info!(
                    "Restart button pressed with type: {:?}",
                    button.restart_type
                );

                match button.restart_type {
                    RestartButtonType::Default => {
                        state.set(StateGame::DefaultRestart);
                        break;
                    }
                    RestartButtonType::GameOver => {
                        state.set(StateGame::GameOverRestart);
                        break;
                    }
                }
            }
        }
    }

    fn create_node_header() -> Node {
        Node {
            width: HEADER_RESTART_BUTTON_WIDTH,
            height: HEADER_RESTART_BUTTON_HEIGHT,
            margin: HEADER_RESTART_BUTTON_MARGIN,
            ..default()
        }
    }

    fn create_node_game_over() -> Node {
        Node {
            max_width: GAME_OVER_RESTART_BUTTON_WIDTH,
            max_height: GAME_OVER_RESTART_BUTTON_HEIGHT,
            justify_content: GAME_OVER_RESTART_BUTTON_JUSTIFY_CONTENT,
            ..default()
        }
    }

    fn create_button(
        node: Node,
        assets: &Res<AssetServer>,
        _type: RestartButtonType,
    ) -> (Node, Button, ImageNode, RestartButton) {
        (
            node,
            Button,
            ImageNode {
                image: assets.load(RESTART_BUTTON_BACK_IMAGE_PATH),
                ..default()
            },
            RestartButton::new(_type),
        )
    }

    fn create_image(assets: &Res<AssetServer>) -> ImageNode {
        ImageNode {
            image: assets.load(RESTART_BUTTON_IMAGE_PATH),
            ..default()
        }
    }
}
