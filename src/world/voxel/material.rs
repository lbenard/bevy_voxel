#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    pub id: u32,
}

pub const GRASS: Material = Material { id: 0 };
pub const DIRT: Material = Material { id: 1 };
pub const STONE: Material = Material { id: 2 };
