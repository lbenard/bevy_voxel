use bevy::prelude::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
}

pub const GRASS: Material = Material {
    color: Color::rgb(0.0, 0.6, 0.1),
};
pub const DIRT: Material = Material {
    color: Color::rgb(0.6, 0.3, 0.1),
};
pub const STONE: Material = Material {
    color: Color::rgb(0.5, 0.5, 0.5),
};
