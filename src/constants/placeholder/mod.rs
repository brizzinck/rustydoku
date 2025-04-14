pub mod animation;
pub mod assets;
pub mod transform;

pub use animation::*;
pub use assets::*;
pub use transform::*;

/// Name used to reference the placeholder entity in the Bevy UI node tree.
pub const PLACEHOLDER_NAME_HIERARCHY: &str = "Placeholder";
