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

impl FigureSpawner {
    pub(crate) fn add_lerp_figure(&mut self, entity: Entity) {
        self.lerp_figures.insert(entity);
    }

    pub(crate) fn remove_lerp_figure(&mut self, entity: Entity) {
        self.lerp_figures.remove(&entity);
    }
}
