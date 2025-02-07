use bevy::prelude::*;

pub fn spawn_button_restart(mut commands: Commands) {
    commands
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                position_type: PositionType::Absolute,
                top: Val::Px(100.),
                right: Val::Px(10.),
                ..default()
            },
            BorderColor(Color::BLACK),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            RestartButton,
        ))
        .with_child((
            Text::new("Button"),
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        ));
}

pub fn handle_restart_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            info!("restart button pressed");
        }
    }
}

#[derive(Component)]
pub struct RestartButton;
