use bevy::prelude::IVec3;

use self::{material::Material, shape::Shape};

pub mod material;
pub mod shape;

// TODO: Maybe split into two structs, a voxel that represent any voxel (shape + material), and a world voxel (absolute position, properties, and shape + material)
#[derive(Copy, Clone, Debug)]
pub struct VoxelDescriptor {
    pub shape: Shape,
    pub material: Material,
}

#[derive(Copy, Clone)]
pub struct Voxel {
    pub position: IVec3,
    pub shape: Shape,
    pub material: Material,
}

impl From<Voxel> for VoxelDescriptor {
    fn from(val: Voxel) -> Self {
        VoxelDescriptor {
            shape: val.shape,
            material: val.material,
        }
    }
}
