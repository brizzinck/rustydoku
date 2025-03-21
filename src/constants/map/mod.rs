pub mod assets;
pub mod transform;

pub use assets::*;
pub use transform::*;

/// Name used in the ECS hierarchy for the entire map node.
pub const MAP_NAME_HIERARCHY: &str = "Map";
