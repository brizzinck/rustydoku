/// The target Y position for the camera in the default (idle) state.
///
/// This is the vertical viewport origin the camera should be at during normal gameplay.
pub const CAMERA_POSITION_Y_IDLE: f32 = 0.5;

/// The target Y position for the camera in the game over state.
///
/// Used to slightly shift the camera upward when the game over panel is displayed.
pub const CAMERA_POSITION_Y_GAME_OVER: f32 = 0.65;

/// The speed at which the camera moves into its default (idle) position.
///
/// Used in linear interpolation (`lerp`) to control the animation speed for returning
/// to the idle camera position.
pub const CAMERA_ANIMATION_IN_POSITION_SPEED: f32 = 2.0;

/// The speed at which the camera moves out to the game over position.
///
/// Used in linear interpolation (`lerp`) to control the animation speed for transitioning
/// to the game over camera position.
pub const CAMERA_ANIMATION_OUT_POSITION_SPEED: f32 = 2.0;
