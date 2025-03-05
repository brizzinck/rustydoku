use crate::components::ui::{
    button_restart::RestartButton,
    header::{score_text::HeaderCurrentScoreText, HeaderUI},
};
use bevy::prelude::*;

pub mod score_text;

impl HeaderUI {
    pub fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
        let parent = commands.spawn(Self::create_header()).id();
        HeaderCurrentScoreText::spawn(&mut commands, parent, &assets);
        RestartButton::spawn_in_header(&mut commands, parent, &assets);
    }

    pub fn hide(mut query: Query<&mut Visibility, With<HeaderUI>>) {
        for mut style in query.iter_mut() {
            *style = Visibility::Hidden;
        }
    }

    pub fn show(mut visibility: Query<&mut Visibility, With<HeaderUI>>) {
        for mut vis in visibility.iter_mut() {
            *vis = Visibility::Inherited;
        }
    }
}
