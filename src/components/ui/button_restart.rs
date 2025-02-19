use crate::constants::ui::assets::*;
use crate::constants::ui::restart_button::*;
use crate::states::gameplay::StateGame;
use bevy::prelude::*;

#[derive(Component)]
pub struct RestartButton;

impl RestartButton {
    pub fn spawn_in_header(commands: &mut Commands, parent: Entity, assets: &Res<AssetServer>) {
        commands
            .spawn(create_restart_button(create_node_header(), assets))
            .with_child(create_image(assets))
            .set_parent(parent);
    }

    pub fn spawn_in_game_over(commands: &mut ChildBuilder, assets: &Res<AssetServer>) {
        commands
            .spawn(create_restart_button(create_node_game_over(), assets))
            .with_child(create_image(assets));
    }

    pub fn handle(
        mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
        mut state: ResMut<NextState<StateGame>>,
    ) {
        for interaction in &mut interaction_query {
            if *interaction == Interaction::Pressed {
                info!("restart button pressed");
                state.set(StateGame::Restart);
                break;
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

fn create_restart_button(
    node: Node,
    assets: &Res<AssetServer>,
) -> (Node, Button, ImageNode, RestartButton) {
    (
        node,
        Button,
        ImageNode {
            image: assets.load(RESTART_BUTTON_BACK_IMAGE_PATH),
            ..default()
        },
        RestartButton,
    )
}

fn create_image(assets: &Res<AssetServer>) -> ImageNode {
    ImageNode {
        image: assets.load(RESTART_BUTTON_IMAGE_PATH),
        ..default()
    }
}
