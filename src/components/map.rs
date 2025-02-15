use bevy::{prelude::*, utils::HashMap};
#[cfg(feature = "debug-inspector")]
use bevy_inspector_egui::prelude::*;
use std::ops::RangeInclusive;

use crate::{resource::map::Map, states::StateGame};

pub const MAP_SIZE: i8 = 9;
pub const TILE_SIZE: f32 = 40.0;
pub const MAP_SPAWN_POS: RangeInclusive<i8> = (-MAP_SIZE / 2)..=(MAP_SIZE / 2);
const COLOR_LIGHT: Color = Color::srgb(0.67, 0.67, 0.67);
const COLOR_DARK: Color = Color::srgb(0.53, 0.53, 0.53);

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
    let parent = commands
        .spawn((
            Name::new("Map"),
            Transform::from_translation(Vec3::ZERO),
            Sprite::default(),
        ))
        .id();

    let mut hash_titles = HashMap::with_capacity(MAP_SIZE as usize * MAP_SIZE as usize);
    for (zero_x, x) in MAP_SPAWN_POS.enumerate() {
        for (zero_y, y) in MAP_SPAWN_POS.enumerate() {
            let color = if ((zero_x / 3) + (zero_y / 3)) % 2 == 0 {
                COLOR_DARK
            } else {
                COLOR_LIGHT
            };

            let transform = Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.);
            let tile = commands
                .spawn((
                    Name::new(format!("Tile ({}, {})", transform.x, transform.y)),
                    Sprite {
                        color,
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        image: assets.load("ferris.png"),
                        ..default()
                    },
                    Transform::from_translation(transform),
                    GlobalTransform::default(),
                    InheritedVisibility::default(),
                    Tile {
                        default_color: color,
                        square: None,
                    },
                ))
                .set_parent(parent)
                .id();

            hash_titles.insert((transform.x as i32, transform.y as i32), tile);
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
