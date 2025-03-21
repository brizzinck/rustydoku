pub mod animation;
pub mod audio;
pub mod interactive;
pub mod transform;

pub use animation::*;
pub use audio::*;
use bevy::math::Vec2;
pub use interactive::*;
pub use transform::*;

pub struct FigureData {
    pub(crate) shape: &'static [Vec2],
    pub(crate) name: &'static str,
    pub(crate) weight: u32,
}

/// Weight is used to determine the probability of the figure being selected
/// the higher the weight, the higher the probability
pub const FIGURES: [FigureData; 12] = [
    FigureData {
        shape: &C,
        name: "C",
        weight: 2,
    },
    FigureData {
        shape: &SQUARE,
        name: "Square",
        weight: 5,
    },
    FigureData {
        shape: &BIG_T,
        name: "Big T",
        weight: 3,
    },
    FigureData {
        shape: &CUBE,
        name: "Cube",
        weight: 4,
    },
    FigureData {
        shape: &LINE_3,
        name: "Line 3",
        weight: 6,
    },
    FigureData {
        shape: &T,
        name: "T",
        weight: 3,
    },
    FigureData {
        shape: &BIG_L,
        name: "Big L",
        weight: 2,
    },
    FigureData {
        shape: &L,
        name: "L",
        weight: 4,
    },
    FigureData {
        shape: &LINE_2,
        name: "Line 2",
        weight: 7,
    },
    FigureData {
        shape: &SMALL_L,
        name: "SMALL L",
        weight: 5,
    },
    FigureData {
        shape: &ZIG_ZAG,
        name: "Zig Zag",
        weight: 4,
    },
    FigureData {
        shape: &BIG_ZIG_ZAG,
        name: "Big Zig Zag",
        weight: 2,
    },
];

const SQUARE: [Vec2; 1] = [Vec2::new(0., 0.)];

const BIG_T: [Vec2; 5] = [
    Vec2::new(-1., 1.),
    Vec2::new(0., 1.),
    Vec2::new(1., 1.),
    Vec2::new(0., 0.),
    Vec2::new(0., -1.),
];

const BIG_L: [Vec2; 5] = [
    Vec2::new(-1., -1.),
    Vec2::new(-1., 0.),
    Vec2::new(-1., 1.),
    Vec2::new(0., -1.),
    Vec2::new(1., -1.),
];

const L: [Vec2; 4] = [
    Vec2::new(-0.5, -1.),
    Vec2::new(-0.5, 0.),
    Vec2::new(-0.5, 1.),
    Vec2::new(0.5, -1.),
];

const SMALL_L: [Vec2; 3] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
];

const CUBE: [Vec2; 4] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
    Vec2::new(0.5, -0.5),
];

const LINE_3: [Vec2; 3] = [Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(-1., 0.)];

const LINE_2: [Vec2; 2] = [Vec2::new(-0.5, 0.), Vec2::new(0.5, 0.)];

const BIG_ZIG_ZAG: [Vec2; 5] = [
    Vec2::new(-1., -1.),
    Vec2::new(-1., 0.),
    Vec2::new(0., 0.),
    Vec2::new(1., 0.),
    Vec2::new(1., 1.),
];

const C: [Vec2; 5] = [
    Vec2::new(-0.5, -1.),
    Vec2::new(-0.5, 0.),
    Vec2::new(0.5, -1.),
    Vec2::new(0.5, 1.),
    Vec2::new(-0.5, 1.),
];

const ZIG_ZAG: [Vec2; 4] = [
    Vec2::new(-0.5, -1.),
    Vec2::new(0.5, 0.),
    Vec2::new(-0.5, 0.),
    Vec2::new(0.5, 1.),
];

const T: [Vec2; 4] = [
    Vec2::new(-1., 0.5),
    Vec2::new(0., 0.5),
    Vec2::new(1., 0.5),
    Vec2::new(0., -0.5),
];
