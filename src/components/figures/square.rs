use super::{spawner::spawn_empty_figure, Figure};
use crate::{
    components::map::{Tile, TILE_SIZE},
    states::StateGame,
};
use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = TILE_SIZE;

#[derive(Component, Default)]
pub struct Square {
    pub(super) parent: Option<Entity>,
}

pub(super) fn spawn(commands: &mut Commands, position: Vec2, assets: &Res<AssetServer>) -> Entity {
    let (parent, rotation) = spawn_empty_figure(commands, position, &[Vec2::new(0., 0.)]);
    let child = spawn_child(commands, parent, Vec2::new(0., 0.), rotation, assets);

    commands.entity(parent).insert(Figure {
        squares: vec![child],
        squares_offset: vec![Vec2::new(0., 0.)],
    });
    commands.entity(parent).insert(Name::new("square"));

    parent
}

pub(super) fn spawn_child(
    commands: &mut Commands,
    parent: Entity,
    position: Vec2,
    rotation: Quat,
    assets: &Res<AssetServer>,
) -> Entity {
    let child = commands
        .spawn((
            Sprite {
                image: assets.load("ferris.png"),
                custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                ..Default::default()
            },
            Transform {
                translation: Vec3::new(position.x * SQUARE_SIZE, position.y * SQUARE_SIZE, 1.0),
                rotation: rotation.inverse(),
                ..Default::default()
            },
            Square {
                parent: Some(parent),
            },
        ))
        .set_parent(parent)
        .id();

    child
}

pub(crate) fn highlight(
    mut tile_query: Query<(&Tile, &mut Sprite, &GlobalTransform, Entity)>,
    figure_query: Query<(&Figure, &Transform)>,
    mut square_query: Query<(Entity, &GlobalTransform, &mut Square)>,
    current_state: Res<State<StateGame>>,
) {
    for (tile, mut sprite, _, _) in tile_query.iter_mut() {
        sprite.color = tile.default_color;
    }

    if let StateGame::Dragging(figure) = current_state.get() {
        let all_tiles = tile_query
            .iter()
            .map(|(tile, _, transform, entity)| (tile, transform, entity))
            .collect::<Vec<_>>();

        let mut highlight_tiles = vec![];
        if let Ok((figure, transform)) = figure_query.get(*figure) {
            if transform.scale.x < 1. {
                return;
            }
            for &square_entity in figure.squares.iter() {
                if let Ok((_, square_transform, _)) = square_query.get_mut(square_entity) {
                    if let Some(tile_entity) = check_for_place(square_transform, &all_tiles) {
                        highlight_tiles.push(tile_entity);
                    }
                }
            }

            if highlight_tiles.len() == figure.squares.len() {
                for tile_entity in highlight_tiles.into_iter() {
                    if let Ok((_, mut sprite, _, _)) = tile_query.get_mut(tile_entity) {
                        sprite.color = Color::srgb(100., 100., 100.);
                    }
                }
            }
        }
    }
}

pub fn check_for_place(
    transofrm: &GlobalTransform,
    tile_query: &Vec<(&Tile, &GlobalTransform, Entity)>,
) -> Option<Entity> {
    let square_pos = transofrm.translation().truncate();
    let square_grid_x = (square_pos.x / TILE_SIZE).round() as i32;
    let square_grid_y = (square_pos.y / TILE_SIZE).round() as i32;

    for (tile, tile_transform, tile_entity) in tile_query.iter() {
        let tile_pos = tile_transform.translation().truncate();
        let tile_grid_x = (tile_pos.x / TILE_SIZE).round() as i32;
        let tile_grid_y = (tile_pos.y / TILE_SIZE).round() as i32;

        if (square_grid_x == tile_grid_x && square_grid_y == tile_grid_y) && tile.square.is_none() {
            return Some(*tile_entity);
        }
    }

    None
}
