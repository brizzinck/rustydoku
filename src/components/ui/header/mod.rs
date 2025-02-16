use super::button_restart::spawn_restart_button_header;
use crate::constants::ui::header::*;
use bevy::prelude::*;
use score::spawn_text_score;

pub mod score;

#[derive(Component)]
pub struct HeaderUI;

pub fn spawn_header(mut commands: Commands, assets: Res<AssetServer>) {
    let parent = commands.spawn(create_header()).id();

    spawn_text_score(&mut commands, parent, &assets);
    spawn_restart_button_header(&mut commands, parent, &assets);
}

pub fn reset_header(visibility: &mut Query<&mut Visibility, With<HeaderUI>>) {
    for mut vis in visibility.iter_mut() {
        *vis = Visibility::Inherited;
    }
}

fn create_header() -> (Node, Visibility, Name, HeaderUI) {
    (
        Node {
            left: HEADER_LEFT,
            top: HEADER_TOP,
            width: HEADER_WIDTH,
            ..default()
        },
        Visibility::Inherited,
        Name::new(HEADER_NAME_HIERARCHY),
        HeaderUI,
    )
}
