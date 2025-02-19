use crate::constants::ui::header::*;
use bevy::prelude::*;

pub mod score_text;

#[derive(Component)]
pub struct HeaderUI;

impl HeaderUI {
    pub(crate) fn create_header() -> (Node, Visibility, Name, HeaderUI) {
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
}
