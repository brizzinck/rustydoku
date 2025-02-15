use crate::states::StateGame;
use bevy::prelude::*;

pub fn spawn_button_restart_top(
    commands: &mut Commands,
    parent: Entity,
    assets: &Res<AssetServer>,
) {
    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                position_type: PositionType::Relative,
                margin: UiRect {
                    right: Val::Px(10.0),
                    top: Val::Px(20.0),
                    left: Val::Auto,
                    bottom: Val::Auto,
                },
                ..default()
            },
            ImageNode {
                image: assets.load("restart_button_back.png"),
                ..default()
            },
            RestartButton,
        ))
        .with_child(ImageNode {
            image: assets.load("restart_button.png"),
            ..default()
        })
        .set_parent(parent);
}

pub fn spawn_button_restart(commands: &mut ChildBuilder, assets: &Res<AssetServer>) {
    commands
        .spawn((
            Button,
            Node {
                max_width: Val::Px(66.0),
                max_height: Val::Px(66.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            RestartButton,
            ImageNode {
                image: assets.load("restart_button_back.png"),
                ..default()
            },
        ))
        .with_child(ImageNode {
            image: assets.load("restart_button.png"),
            ..default()
        });
}

pub fn handle_restart_button(
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

#[derive(Component)]
pub struct RestartButton;
