use crate::{
    components::{
        figures::{spawner::random_spawn_figure, Figure},
        ui::header::HeaderUI,
    },
    constants::figure::{placeholder::*, *},
    events::figure::FigureTriggerUp,
    states::gameplay::StateGame,
};
use assets::FIGURE_PLACEHOLDER_IMAGE_PATH;
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

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

pub fn removig_lerp_figures(
    mut event_reader: EventReader<FigureTriggerUp>,
    mut figure_spawner: ResMut<FigureSpawner>,
) {
    for FigureTriggerUp(entity) in event_reader.read() {
        figure_spawner.remove_lerp_figure(*entity);
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
                transform.translation = transform.translation.lerp(
                    *position,
                    time.delta_secs() * FIGURE_SPEED_RETURN_TO_PLACEHOLDER,
                );

                if transform.translation.distance(*position) < 0.01 {
                    transform.translation = *position;
                } else {
                    remove = false;
                }
            }
            transform.scale = transform.scale.lerp(
                Vec3::splat(PLACEHOLDER_SCALE),
                time.delta_secs() * FIGURE_SPEED_RETURN_TO_PLACEHOLDER,
            );

            if transform.scale.distance(Vec3::splat(PLACEHOLDER_SCALE)) < 0.01 {
                transform.scale = Vec3::splat(PLACEHOLDER_SCALE);
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
            Transform::from_translation(Vec3::ZERO),
            Visibility::Inherited,
            FigureZone,
        ))
        .id();

    for &position in PLACEHOLDER_POSITION.iter() {
        commands.entity(parent).with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: assets.load(FIGURE_PLACEHOLDER_IMAGE_PATH),
                    custom_size: Some(Vec2::new(PLACEHOLDER_SIZE, PLACEHOLDER_SIZE)),
                    color: PLACEHOLDER_COLOR.into(),
                    ..default()
                },
                Transform::from_translation(Vec3::new(
                    position.0,
                    position.1,
                    PLACEHOLDER_POSITION_Z,
                )),
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
        for &position in PLACEHOLDER_POSITION.iter() {
            figure_spawner.figures.insert(
                random_spawn_figure(&mut commands, Vec2::new(position.0, position.1), &assets),
                Vec3::new(position.0, position.1, FIGURE_POSITION_Z),
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
