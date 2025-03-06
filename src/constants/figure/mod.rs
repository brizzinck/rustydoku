pub mod animation;
pub mod interactive;
pub mod transform;

pub use animation::*;
use bevy::math::Vec2;
pub use interactive::*;
pub use transform::*;

pub const FIGURES: [(&[Vec2], &str); 13] = [
    (&C, C_NAME),
    (&SQUARE, SQUARE_NAME),
    (&BIG_T, BIG_T_NAME),
    (&CUBE, CUBE_NAME),
    (&LINE_3, LINE_3_NAME),
    (&T, T_NAME),
    (&BIG_L, BIG_L_NAME),
    (&L, L_NAME),
    (&LINE_2, LINE_2_NAME),
    (&LINE_3, LINE_3_NAME),
    (&SMALL_L, SMALL_L_NAME),
    (&ZIG_ZAG, ZIG_ZAG_NAME),
    (&BIG_ZIG_ZAG, BIG_ZIG_ZAG_NAME),
];

const SQUARE: [Vec2; 1] = [Vec2::new(0., 0.)];
const SQUARE_NAME: &str = "Square";

const BIG_T: [Vec2; 5] = [
    Vec2::new(-1., 1.),
    Vec2::new(0., 1.),
    Vec2::new(1., 1.),
    Vec2::new(0., 0.),
    Vec2::new(0., -1.),
];
const BIG_T_NAME: &str = "Big T";

const BIG_L: [Vec2; 5] = [
    Vec2::new(-1., -1.),
    Vec2::new(-1., 0.),
    Vec2::new(-1., 1.),
    Vec2::new(0., -1.),
    Vec2::new(1., -1.),
];
const BIG_L_NAME: &str = "Big L";

const L: [Vec2; 4] = [
    Vec2::new(-0.5, -1.),
    Vec2::new(-0.5, 0.),
    Vec2::new(-0.5, 1.),
    Vec2::new(0.5, -1.),
];
const L_NAME: &str = "L";

const SMALL_L: [Vec2; 3] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
];
const SMALL_L_NAME: &str = "SMALL L";

const CUBE: [Vec2; 4] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
    Vec2::new(0.5, -0.5),
];
const CUBE_NAME: &str = "Cube";

const LINE_3: [Vec2; 3] = [Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(-1., 0.)];
const LINE_3_NAME: &str = "Horizontal Line 3";

const LINE_2: [Vec2; 2] = [Vec2::new(-0.5, 0.), Vec2::new(0.5, 0.)];
const LINE_2_NAME: &str = "Horizontal Line 2";

const BIG_ZIG_ZAG: [Vec2; 5] = [
    Vec2::new(-1., -1.),
    Vec2::new(-1., 0.),
    Vec2::new(0., 0.),
    Vec2::new(1., 0.),
    Vec2::new(1., 1.),
];
const BIG_ZIG_ZAG_NAME: &str = "Big Zig Zag";

const C: [Vec2; 5] = [
    Vec2::new(-0.5, -1.),
    Vec2::new(-0.5, 0.),
    Vec2::new(0.5, -1.),
    Vec2::new(0.5, 1.),
    Vec2::new(-0.5, 1.),
];
const C_NAME: &str = "C";

const ZIG_ZAG: [Vec2; 4] = [
    Vec2::new(-0.5, -1.),
    Vec2::new(0.5, 0.),
    Vec2::new(-0.5, 0.),
    Vec2::new(0.5, 1.),
];
const ZIG_ZAG_NAME: &str = "Zig Zag";

const T: [Vec2; 4] = [
    Vec2::new(-1., 0.5),
    Vec2::new(0., 0.5),
    Vec2::new(1., 0.5),
    Vec2::new(0., -0.5),
];
const T_NAME: &str = "T";
