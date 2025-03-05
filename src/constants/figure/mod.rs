pub mod animation;
pub mod interactive;
pub mod transform;

pub use animation::*;
use bevy::math::Vec2;
pub use interactive::*;
pub use transform::*;

pub const FIGURES: [(&[Vec2], &str); 5] = [
    (&SQUARE, SQUARE_NAME),
    (&BIG_T_FIGURE, BIG_T_FIGURE_NAME),
    (&CUBE, CUBE_NAME),
    (&LINE, LINE_NAME),
    (&T_FIGURE, T_FIGURE_NAME),
];

const SQUARE: [Vec2; 1] = [Vec2::new(0., 0.)];
const SQUARE_NAME: &str = "Square";

const BIG_T_FIGURE: [Vec2; 5] = [
    Vec2::new(-1.0, 1.),
    Vec2::new(0.0, 1.),
    Vec2::new(1.0, 1.),
    Vec2::new(0.0, -0.),
    Vec2::new(0.0, -1.),
];
const BIG_T_FIGURE_NAME: &str = "Big T";

const CUBE: [Vec2; 4] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
    Vec2::new(0.5, -0.5),
];
const CUBE_NAME: &str = "Cube";

const LINE: [Vec2; 3] = [Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(-1., 0.)];
const LINE_NAME: &str = "Line";

const T_FIGURE: [Vec2; 4] = [
    Vec2::new(-1.0, 0.5),
    Vec2::new(0.0, 0.5),
    Vec2::new(1.0, 0.5),
    Vec2::new(0.0, -0.5),
];
const T_FIGURE_NAME: &str = "T";
