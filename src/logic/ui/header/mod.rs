use crate::components::ui::{
    buttons::ButtonsPanel,
    header::{score_text::HeaderCurrentScoreTextComponent, HeaderUI},
};
use bevy::prelude::*;

pub mod score_text;

impl HeaderUI {
    pub fn spawn(mut commands: Commands, assets: Res<AssetServer>) {
        commands
            .spawn(Self::create_header())
            .with_children(|header| {
                HeaderCurrentScoreTextComponent::spawn(header, &assets);
                ButtonsPanel::spawn_header(header, &assets);
            });
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
