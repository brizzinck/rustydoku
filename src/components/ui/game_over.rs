use bevy::prelude::*;

pub struct GameOverPlugin;

fn _spawn_game_over_ui(mut commands: Commands) {
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(25.),
            right: Val::Percent(25.),
            top: Val::Percent(30.),
            bottom: Val::Percent(30.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER: "),
                TextFont {
                    font_size: 57.0,
                    ..default()
                },
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        margin: UiRect::all(Val::Px(10.)),
                        padding: UiRect::all(Val::Px(10.)),
                        border: UiRect::all(Val::Px(2.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    RestartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Restart: "),
                        TextFont {
                            font_size: 57.0,
                            ..default()
                        },
                    ));
                });
        });
}

#[derive(Component)]
#[allow(dead_code)]
struct RestartButton;

fn _cleanup_game_over_ui(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
