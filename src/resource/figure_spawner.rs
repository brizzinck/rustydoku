use crate::constants::{
    placeholder::*,
    square::{SQAURE_IMAGE_DEFAULT_PATH, SQUARE_IMAGE_HIGHLIGHT_PATH},
};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Resource, Default)]
pub struct FigureSpawnerResource {
    pub(crate) figures: HashMap<Entity, Vec3>,
    pub(crate) lerp_figures: HashSet<Entity>,
    pub(crate) bounce_figures: HashSet<Entity>,
    pub placeholder_image: Handle<Image>,
    pub placeholder_image_deactive: Handle<Image>,
    pub square_image: Handle<Image>,
    pub square_image_highlighted: Handle<Image>,
}

impl FigureSpawnerResource {
    pub(crate) fn init(assets: &AssetServer) -> FigureSpawnerResource {
        let placeholder_image = assets.load(FIGURE_PLACEHOLDER_IMAGE_DEFAULT);
        let placeholder_image_deactive = assets.load(FIGURE_PLACEHOLDER_IMAGE_RED);
        let square_image = assets.load(SQAURE_IMAGE_DEFAULT_PATH);
        let square_image_highlighted = assets.load(SQUARE_IMAGE_HIGHLIGHT_PATH);

        FigureSpawnerResource {
            placeholder_image,
            square_image,
            placeholder_image_deactive,
            square_image_highlighted,
            ..Default::default()
        }
    }

    pub(crate) fn get_square_image_highlighted(&self) -> Handle<Image> {
        self.square_image_highlighted.clone()
    }

    pub(crate) fn get_placeholder_image(&self) -> Handle<Image> {
        self.placeholder_image.clone()
    }

    pub(crate) fn get_placeholder_image_deactive(&self) -> Handle<Image> {
        self.placeholder_image_deactive.clone()
    }

    pub(crate) fn get_square_image(&self) -> Handle<Image> {
        self.square_image.clone()
    }

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
