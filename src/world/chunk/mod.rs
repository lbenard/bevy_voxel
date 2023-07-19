use bevy::prelude::{Component, IVec3, UVec3};
use ndshape::Shape as NdShape;

use super::voxel::VoxelDescriptor;

pub mod generator;
pub mod loader;
pub mod material;
pub mod mesh;

pub const CHUNK_SIZE: UVec3 = UVec3::new(64, 64, 64);
pub type Shape = ndshape::ConstShape3u32<64, 64, 64>;

#[derive(Component)]
pub struct ChunkMarker;

#[derive(Component, PartialEq, Clone, Copy, Eq, Hash)]
pub struct ChunkCoordinates(pub IVec3);

/// Describe the chunk loading state.
/// A chunk is `Loading` at creation, `Loaded` when generated but not displayed, and `Rendered` when generated and displayed.
/// Any `Unloaded` chunk will get deleted from memory.
#[derive(Component)]
pub enum ChunkState {
    // Unloaded,
    Loading,
    // Loaded,
    Rendered,
}

pub type VoxelIndex = u8;

pub struct Grid {
    pub size: UVec3,
    pub voxels: Vec<Option<VoxelDescriptor>>,
    shape: ndshape::ConstShape3u32<64, 64, 64>,
}

impl Grid {
    pub fn voxel_at_pos(&self, pos: IVec3) -> Option<VoxelDescriptor> {
        // let pos = pos + IVec3::ONE;
        let idx = self.pos_idx(pos.as_uvec3());
        if idx >= self.voxels.len() {
            return None;
        }
        self.voxels[idx]
    }

    pub fn pos_idx(&self, pos: UVec3) -> usize {
        self.shape.linearize(pos.to_array()) as usize
    }
}
