use crate::states::StateGame;
use bevy::prelude::*;

pub fn spawn_button_restart_top(mut commands: Commands) {
    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                border: UiRect::all(Val::Px(1.0)),
                position_type: PositionType::Absolute,
                top: Val::Px(10.),
                right: Val::Px(10.),
                ..default()
            },
            BorderColor(Color::BLACK),
            BackgroundColor(Color::srgb(0.85, 0.15, 0.15)),
            RestartButton,
        ))
        .with_child(ImageNode::default());
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
