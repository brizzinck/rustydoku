use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Resource, Default)]
pub struct FigureSpawner {
    pub(crate) figures: HashMap<Entity, Vec3>,
    pub(crate) lerp_figures: HashSet<Entity>,
    pub(crate) bounce_figures: HashSet<Entity>,
}

impl FigureSpawner {
    pub(crate) fn add_lerp_figure(&mut self, entity: Entity) {
        self.remove_upscaling_figure(entity);
        self.lerp_figures.insert(entity);
    }

    pub(crate) fn remove_lerp_figure(&mut self, entity: Entity) {
        self.lerp_figures.remove(&entity);
    }

    pub(crate) fn add_upscaling_figure(&mut self, entity: Entity) {
        self.remove_lerp_figure(entity);
        self.bounce_figures.insert(entity);
    }

    pub(crate) fn remove_upscaling_figure(&mut self, entity: Entity) {
        self.bounce_figures.remove(&entity);
    }
}
