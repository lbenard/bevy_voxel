use bevy::{math::UVec3, prelude::IVec3};

use self::{material::Material, shape::Shape};

pub mod material;
pub mod shape;

// TODO: Maybe split into two structs, a voxel that represent any voxel (shape + material), and a world voxel (absolute position, properties, and shape + material)
#[derive(Copy, Clone, Debug)]
pub struct VoxelDescriptor {
    pub shape: Shape,
    pub material: Material,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    North,
    South,
    Top,
    Bottom,
    West,
    East,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Side::North => Side::South,
            Side::South => Side::North,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::West => Side::East,
            Side::East => Side::West,
        }
    }

    pub fn adjacent_position(&self, pos: UVec3) -> IVec3 {
        let pos = pos.as_ivec3();
        match self {
            Side::North => IVec3 {
                x: pos.x,
                y: pos.y,
                z: pos.z + 1,
            },
            Side::South => IVec3 {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            },
            Side::Top => IVec3 {
                x: pos.x,
                y: pos.y + 1,
                z: pos.z,
            },
            Side::Bottom => IVec3 {
                x: pos.x,
                y: pos.y - 1,
                z: pos.z,
            },
            Side::West => IVec3 {
                x: pos.x + 1,
                y: pos.y,
                z: pos.z,
            },
            Side::East => IVec3 {
                x: pos.x - 1,
                y: pos.y,
                z: pos.z,
            },
        }
    }
}
