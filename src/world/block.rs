use bevy::prelude::IVec3;

use super::terrain::block_descriptor::{material::Material, shape::Shape, BlockDescriptor};

#[derive(Copy, Clone)]
pub struct Block {
    pub position: IVec3,
    pub shape: Shape,
    pub material: Material,
}

impl Into<BlockDescriptor> for Block {
    fn into(self) -> BlockDescriptor {
        BlockDescriptor {
            shape: self.shape,
            material: self.material,
        }
    }
}
