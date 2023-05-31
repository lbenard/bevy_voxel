use bevy::prelude::Color;

#[derive(Clone, Copy, PartialEq)]
pub struct Material {
    pub id: u32,
    pub color: Color,
}

pub const GRASS: Material = Material {
    id: 0,
    color: Color::rgb(0.0, 0.6, 0.1),
};
pub const DIRT: Material = Material {
    id: 1,
    color: Color::rgb(0.6, 0.3, 0.1),
};
pub const STONE: Material = Material {
    id: 2,
    color: Color::rgb(0.5, 0.5, 0.5),
};
