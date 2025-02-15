use super::button_restart::spawn_button_restart;
use crate::resource::score::Score;
use bevy::prelude::*;

pub struct GameOverPlugin;

pub fn spawn_game_over_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    score.max_value = score.value.max(score.max_value);

    commands
        .spawn((
            Node {
                left: Val::Auto,
                right: Val::Auto,
                bottom: Val::Auto,
                position_type: PositionType::Absolute,
                top: Val::Percent(120.0),
                width: Val::Percent(132.0),
                height: Val::Percent(26.0),
                margin: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    ..default()
                },
                max_width: Val::Px(500.0),
                max_height: Val::Px(255.0),
                justify_self: JustifySelf::Center,
                ..default()
            },
            GameOverPanel {
                timer: Timer::from_seconds(1.5, TimerMode::Once),
                speed: 1.0,
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Relative,
                        width: Val::Percent(60.0),
                        height: Val::Percent(125.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Auto),
                        row_gap: Val::Percent(5.),
                        ..default()
                    },
                    ImageNode {
                        image: assets.load("restart_panel_background.png"),
                        color: (Srgba::new(1.0, 1.0, 1.0, 0.86).into()),
                        ..default()
                    },
                ))
                .with_children(|panel| {
                    panel.spawn((
                        Text::new("LOSSES"),
                        TextFont {
                            font_size: 37.0,
                            font: assets.load("rusty.otf"),
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.1, 0.1)),
                    ));

                    panel.spawn((
                        Text::new(format!("MAX SCORE: {}", score.max_value)),
                        TextFont {
                            font_size: 27.0,
                            font: assets.load("rusty.otf"),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    panel.spawn((
                        Text::new(format!("SCORE: {}", score.value)),
                        TextFont {
                            font_size: 27.0,
                            font: assets.load("rusty.otf"),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    spawn_button_restart(panel, &assets);
                });
        });
}

#[derive(Component)]
pub struct GameOverPanel {
    pub timer: Timer,
    pub speed: f32,
}

pub fn despawn_game_over_panel(commands: &mut Commands, query: Query<Entity, With<GameOverPanel>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
