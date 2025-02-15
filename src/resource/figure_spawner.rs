use crate::{
    components::{
        figures::{spawner::random_spawn_figure, Figure},
        map::TILE_SIZE,
        ui::header::HeaderUI,
    },
    states::StateGame,
};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

const SIZE: usize = 3;
const FIGURE_POSITIONS: [(f32, f32); SIZE] = [
    (TILE_SIZE * 3.45, TILE_SIZE * -7.),
    (0., TILE_SIZE * -7.),
    (TILE_SIZE * -3.45, TILE_SIZE * -7.),
];

#[derive(Resource, Default)]
pub struct FigureSpawner {
    pub figures: HashMap<Entity, Vec3>,
    pub lerp_figures: HashSet<Entity>,
    pub zone: Option<Entity>,
}

#[derive(Component)]
pub struct FigureZone;

impl FigureSpawner {
    pub fn add_lerp_figure(&mut self, entity: Entity) {
        self.lerp_figures.insert(entity);
    }

    pub fn remove_lerp_figure(&mut self, entity: Entity) {
        self.lerp_figures.remove(&entity);
    }
}

pub fn lerping_figures(
    mut figure_spawner: ResMut<FigureSpawner>,
    mut figures: Query<(&Figure, &mut Transform)>,
    time: Res<Time>,
) {
    let mut to_remove = Vec::new();
    for entity in figure_spawner.lerp_figures.iter() {
        if let Ok((_figure, mut transform)) = figures.get_mut(*entity) {
            let mut remove = true;
            if let Some(position) = figure_spawner.figures.get(entity) {
                transform.translation = transform
                    .translation
                    .lerp(*position, time.delta_secs() * 8.0);

                if transform.translation.distance(*position) < 0.01 {
                    transform.translation = *position;
                } else {
                    remove = false;
                }
            }
            transform.scale = transform
                .scale
                .lerp(Vec3::splat(0.6), time.delta_secs() * 8.0);

            if transform.scale.distance(Vec3::splat(0.6)) < 0.01 {
                transform.scale = Vec3::splat(0.6);
            } else {
                remove = false;
            }

            if remove {
                to_remove.push(*entity);
            }
        }
    }

    figure_spawner
        .lerp_figures
        .retain(|entity| !to_remove.contains(entity));
}

pub fn spawn_zone_figures(mut commands: Commands, assets: Res<AssetServer>) {
    let parent = commands
        .spawn((
            Name::new("Figure Zone"),
            Transform::from_translation(Vec3::new(0., 0., 0.)),
            Visibility::Inherited,
            FigureZone,
        ))
        .id();

    for &position in FIGURE_POSITIONS.iter() {
        commands.entity(parent).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: assets.load("figure_place.png"),
                    custom_size: Some(Vec2::new(
                        (TILE_SIZE * 3. + 10.) * 0.6,
                        (TILE_SIZE * 3. + 10.) * 0.6,
                    )),
                    color: Srgba::new(1., 1., 1., 0.45).into(),
                    ..default()
                },
                Transform::from_translation(Vec3::new(position.0, position.1, 0.)),
            ));
        });
    }
}

pub fn spawn_figures(
    mut commands: Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    assets: Res<AssetServer>,
) {
    if figure_spawner.figures.is_empty() {
        for &position in FIGURE_POSITIONS.iter() {
            figure_spawner.figures.insert(
                random_spawn_figure(&mut commands, Vec2::new(position.0, position.1), &assets),
                Vec3::new(position.0, position.1, 1.),
            );
        }
    }
}

pub fn despawn_figures(
    mut commands: Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    state: Res<State<StateGame>>,
    mut next_state: ResMut<NextState<StateGame>>,
    assets: Res<AssetServer>,
) {
    if let StateGame::Placed(entity) = state.get() {
        commands.entity(*entity).despawn();
        figure_spawner.figures.remove(entity);
        spawn_figures(commands, figure_spawner, assets);
        next_state.set(StateGame::CheckCombo);
    }
}

pub fn restart_figures(
    commands: &mut Commands,
    mut figure_spawner: ResMut<FigureSpawner>,
    assets: Res<AssetServer>,
) {
    for (entity, _) in figure_spawner.figures.iter() {
        commands.entity(*entity).despawn_recursive();
    }
    figure_spawner.figures.clear();
    spawn_figures(commands.reborrow(), figure_spawner, assets);
}

pub fn clear_figures(mut commands: Commands, mut figure_spawner: ResMut<FigureSpawner>) {
    for (entity, _) in figure_spawner.figures.iter() {
        commands.entity(*entity).despawn_recursive();
    }
    figure_spawner.figures.clear();
}

pub fn hidden_figures(
    mut visibility: Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>,
) {
    for mut vis in visibility.iter_mut() {
        *vis = Visibility::Hidden;
    }
}

pub fn show_figures(
    visibility: &mut Query<&mut Visibility, (With<FigureZone>, Without<HeaderUI>)>,
) {
    for mut vis in visibility.iter_mut() {
        *vis = Visibility::Visible;
    }
}
