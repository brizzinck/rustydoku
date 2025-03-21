use crate::constants::{
    placeholder::*,
    square::{SQAURE_IMAGE_DEFAULT_PATH, SQUARE_IMAGE_HIGHLIGHT_PATH},
};
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

/// A resource that manages the spawning, animation, and image assets of figures.
///
/// This resource tracks figures and their visual states (e.g., upscaling or returning to their
/// placeholders) and provides access to image handles used in the UI.
///
/// It includes:
/// - A map of figures and their target positions.
/// - Sets of figures undergoing animations (lerping or upscaling).
/// - Cached handles for placeholder and square images to avoid reloading them repeatedly.
#[derive(Resource, Default)]
pub struct FigureSpawnerResource {
    /// The figures and their positions
    pub(crate) figures: HashMap<Entity, Vec3>,

    /// The figures that are returning to their placeholder
    pub(crate) lerp_figures: HashSet<Entity>,

    /// The figures that upscaling when a player is dragging them
    pub(crate) upscaling_figures: HashSet<Entity>,

    /// The placeholder image for the default state
    placeholder_image: Handle<Image>,

    /// The placeholder image for the placeholder when the figure cannot be placed
    placeholder_image_deactive: Handle<Image>,

    /// The square image for the square
    square_image: Handle<Image>,

    /// The square image for the square when figure can be placed on it
    square_image_highlighted: Handle<Image>,
}

impl FigureSpawnerResource {
    /// Initializes the figure spawner resource with preloaded images for figures and placeholders.
    ///
    /// This loads:
    /// - Default placeholder image
    /// - Deactivated placeholder image
    /// - Default square image
    /// - Highlighted square image
    ///
    /// # Parameters
    /// - `assets`: Reference to the [`AssetServer`] for loading image assets.
    ///
    /// # Returns
    /// - A fully initialized [`FigureSpawnerResource`] with default values and loaded image handles.
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

    /// Returns the image handle for a highlighted square (used for UI feedback).
    ///
    /// # Returns
    /// - [`Handle<Image>`] for the highlighted square image.
    pub(crate) fn get_square_image_highlighted(&self) -> Handle<Image> {
        self.square_image_highlighted.clone()
    }

    /// Returns the image handle for the default figure placeholder.
    ///
    /// # Returns
    /// - [`Handle<Image>`] for the default placeholder.
    pub(crate) fn get_placeholder_image(&self) -> Handle<Image> {
        self.placeholder_image.clone()
    }

    /// Returns the image handle for the deactivated figure placeholder (e.g., when placement is invalid).
    ///
    /// # Returns
    /// - [`Handle<Image>`] for the deactivated placeholder.
    pub(crate) fn get_placeholder_image_deactive(&self) -> Handle<Image> {
        self.placeholder_image_deactive.clone()
    }

    /// Returns the image handle for the default square image (non-highlighted).
    ///
    /// # Returns
    /// - [`Handle<Image>`] for the square image.
    pub(crate) fn get_square_image(&self) -> Handle<Image> {
        self.square_image.clone()
    }

    /// Adds a figure to the lerp set, which indicates it should animate back to its placeholder.
    ///
    /// If the figure was in the upscaling set, it will be removed from there.
    ///
    /// # Parameters
    /// - `entity`: The entity of the figure to add.
    pub(crate) fn add_lerp_figure(&mut self, entity: Entity) {
        self.remove_upscaling_figure(entity);
        self.lerp_figures.insert(entity);
    }

    /// Removes a figure from the lerp set when its return animation is completed.
    ///
    /// # Parameters
    /// - `entity`: The entity of the figure to remove.
    pub(crate) fn remove_lerp_figure(&mut self, entity: Entity) {
        self.lerp_figures.remove(&entity);
    }

    /// Adds a figure to the upscaling set, indicating it is currently being dragged and should scale up.
    ///
    /// If the figure was in the lerp set, it will be removed from there.
    ///
    /// # Parameters
    /// - `entity`: The entity of the figure to add.
    pub(crate) fn add_upscaling_figure(&mut self, entity: Entity) {
        self.remove_lerp_figure(entity);
        self.upscaling_figures.insert(entity);
    }

    /// Removes a figure from the upscaling set after the drag or animation completes.
    ///
    /// # Parameters
    /// - `entity`: The entity of the figure to remove.
    pub(crate) fn remove_upscaling_figure(&mut self, entity: Entity) {
        self.upscaling_figures.remove(&entity);
    }
}
