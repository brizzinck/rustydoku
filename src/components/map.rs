use assets::TILE_IMAGE_PATH;
use bevy::{prelude::*, utils::HashMap};
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;

use crate::{
    constants::{figure::MAX_FIGURE_USIZE, map::*},
    resource::map::Map,
    states::StateGame,
};

#[derive(Component, Default)]
#[cfg_attr(feature = "debug-inspector", derive(Reflect, InspectorOptions))]
#[cfg_attr(feature = "debug-inspector", reflect(Component, InspectorOptions))]
pub struct Tile {
    pub(crate) default_color: Color,
    pub(crate) square: Option<Entity>,
}

pub(crate) fn generate_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    mut next_state: ResMut<NextState<StateGame>>,
    assets: Res<AssetServer>,
) {
    let parent = commands.spawn(create_map()).id();

    let mut hash_titles = HashMap::with_capacity(MAP_SIZE as usize * MAP_SIZE as usize);
    for (zero_x, x) in MAP_SPAWN_POS.enumerate() {
        for (zero_y, y) in MAP_SPAWN_POS.enumerate() {
            let color = if ((zero_x / MAX_FIGURE_USIZE) + (zero_y / MAX_FIGURE_USIZE)) % 2 == 0 {
                COLOR_DARK
            } else {
                COLOR_LIGHT
            };

            let position = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TILE_POSITION_Z);
            let tile = commands
                .spawn(create_tile(color, position, &assets))
                .set_parent(parent)
                .id();

            hash_titles.insert((x as i32, y as i32), tile);
        }
    }

    map.0 = hash_titles;

    next_state.set(StateGame::Idle);
}

pub fn restart_tiles(commands: &mut Commands, mut tiles: Query<&mut Tile>) {
    for mut tile in tiles.iter_mut() {
        if let Some(square) = tile.square {
            tile.square = None;
            commands.entity(square).despawn();
        }
    }
}

fn create_map() -> impl Bundle {
    (
        Name::new(MAP_NAME_HIERARCHY),
        Transform::from_translation(Vec3::ZERO),
        InheritedVisibility::default(),
    )
}

fn create_tile(sprite_color: Color, position: Vec3, assets: &Res<AssetServer>) -> impl Bundle {
    (
        Name::new(format!("Tile ({}, {})", position.x, position.y)),
        Sprite {
            color: sprite_color,
            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
            image: assets.load(TILE_IMAGE_PATH),
            ..default()
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
        InheritedVisibility::default(),
        Tile {
            default_color: sprite_color,
            square: None,
        },
    )
}
