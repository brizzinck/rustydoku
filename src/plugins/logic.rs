use crate::logic::check_combo::check_combination;
use bevy::prelude::*;

pub struct RustydokuLogicPlugin;

impl Plugin for RustydokuLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, check_combination);
    }
}
