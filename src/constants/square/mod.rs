pub mod animation;
pub mod assets;
pub mod transform;

pub use animation::*;
pub use assets::*;
pub use transform::*;

/// UI hierarchy name used to identify squares in the ECS world.
pub const SQUARE_NAME_HIERARCHY: &str = "square";
