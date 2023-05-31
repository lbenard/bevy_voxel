use bevy::prelude::IVec3;

use super::terrain::block_descriptor::{material::Material, shape::Shape};

#[derive(Copy, Clone)]
pub struct Block {
    pub position: IVec3,
    pub shape: Shape,
    pub material: Material,
}
