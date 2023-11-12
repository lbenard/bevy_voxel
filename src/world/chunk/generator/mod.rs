use crate::world::voxel::shape::Shape;

use super::{Terrain, VoxelIndex};

// SHape generators
pub mod height_noise_terrain;
pub mod noise_terrain_generator;

// Materializators
pub mod default_materializator;

// Terrain gen goes into three different phases: shape, materialization and decoration

pub trait TerrainGenerator {
    fn generate(&self, shape: super::Shape) -> Grid;

    fn voxel_idx(bits: &[bool; 8]) -> VoxelIndex {
        let mut idx = 0;
        idx |= bits[0] as u8;
        idx |= (bits[1] as u8) << 1;
        idx |= (bits[2] as u8) << 2;
        idx |= (bits[3] as u8) << 3;
        idx |= (bits[4] as u8) << 4;
        idx |= (bits[5] as u8) << 5;
        idx |= (bits[6] as u8) << 6;
        idx |= (bits[7] as u8) << 7;
        idx
    }
}

pub struct Grid {
    shape: super::Shape,
    pub data: Vec<Shape>,
}

pub trait Materializator {
    fn materialize(&self, chunk: &Grid) -> Terrain;
}
