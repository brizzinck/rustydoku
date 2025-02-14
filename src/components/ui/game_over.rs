use bevy::prelude::*;

use super::button_restart::spawn_button_restart;

pub struct GameOverPlugin;

pub fn spawn_game_over_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(25.),
                right: Val::Percent(25.),
                top: Val::Percent(30.),
                bottom: Val::Percent(30.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Srgba::new(0.0, 0.0, 0.0, 0.5).into()),
            GameOverPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("YOU CAN'T PLACE NEW RUSTY :("),
                TextFont {
                    font_size: 27.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_button_restart(parent);
        });
}

#[derive(Component)]
pub struct GameOverPanel;

pub fn despawn_game_over_panel(commands: &mut Commands, query: Query<Entity, With<GameOverPanel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
