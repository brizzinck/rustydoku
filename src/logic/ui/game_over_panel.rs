use crate::components::ui::header::HeaderUI;
use bevy::prelude::*;

pub fn hide_header(mut query: Query<&mut Visibility, With<HeaderUI>>) {
    for mut style in query.iter_mut() {
        *style = Visibility::Hidden;
    }
}
