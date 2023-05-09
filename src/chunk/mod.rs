use bevy::prelude::{Component, IVec2, IVec3, UVec3};

use crate::terrain::{block::Block, generator::ChunkGenerator};

pub mod generators;

pub const CHUNK_SIZE: UVec3 = UVec3::new(64, 64, 64);

#[derive(Component)]
pub struct Chunk; // {
                  // pub coordinates: IVec3,
                  // pub blocks
                  // }

#[derive(Component, PartialEq)]
pub struct ChunkCoordinates(pub IVec2);

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

pub type BlockIndex = u8;

pub struct Grid {
    pub size: UVec3,
    pub blocks: Vec<Option<Block>>,
}

impl Grid {
    pub fn new(size: UVec3) -> Self {
        let capacity = size.x * size.y * size.z;
        Grid {
            size,
            blocks: vec![None; capacity as usize],
        }
    }

    pub fn generate(mut self, origin: IVec3, generator: &impl ChunkGenerator) -> Self {
        generator.generate(origin, &mut self);
        self
    }

    pub fn block_at_pos(&self, pos: UVec3) -> Option<Block> {
        self.blocks[self.pos_idx(pos)]
    }

    pub const fn pos_idx(&self, pos: UVec3) -> usize {
        ((self.size.z * self.size.x * pos.y) + (self.size.z * pos.x) + pos.z) as usize
    }
}
