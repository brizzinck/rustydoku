use bevy::prelude::*;

use super::{button_restart::spawn_button_restart_top, score::spawn_text_score};

#[derive(Component)]
pub struct HeaderUI;

pub fn spawn_header(mut commands: Commands, assets: Res<AssetServer>) {
    let parent = commands
        .spawn((
            Node {
                left: Val::Percent(2.5),
                top: Val::Px(10.),
                width: Val::Vw(95.),
                height: Val::Vh(10.),
                ..Default::default()
            },
            ImageNode {
                image: assets.load("figure_place.png".to_string()),
                color: Srgba::new(1.0, 1.0, 1.0, 0.0).into(),
                ..Default::default()
            },
            Name::new("Header UI"),
            HeaderUI,
        ))
        .id();

    spawn_text_score(&mut commands, parent, &assets);
    spawn_button_restart_top(&mut commands, parent, &assets);
}

pub fn reset_header(visibility: &mut Query<&mut Visibility, With<HeaderUI>>) {
    for mut vis in visibility.iter_mut() {
        *vis = Visibility::Inherited;
    }
}
