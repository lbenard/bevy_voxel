use bevy::prelude::{Component, IVec3, UVec3};

use crate::terrain::{block::Block, generator::ChunkGenerator};

pub mod generators;
pub mod mesh;

pub const CHUNK_SIZE: UVec3 = UVec3::new(32, 128, 32);

#[derive(Component)]
pub struct Chunk;

#[derive(Component, PartialEq)]
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

pub type BlockIndex = u8;

pub struct Grid {
    pub size: UVec3,
    pub extended_size: UVec3,
    pub blocks: Vec<Option<Block>>,
}

impl Grid {
    pub fn new(size: UVec3) -> Self {
        let extended_size = size + UVec3::new(2, 2, 2);
        let capacity = extended_size.x * extended_size.y * extended_size.z;
        Grid {
            size,
            extended_size,
            blocks: vec![None; capacity as usize],
        }
    }

    pub fn generate(mut self, origin: IVec3, generator: &impl ChunkGenerator) -> Self {
        generator.generate(origin, &mut self);
        self
    }

    pub fn block_at_pos(&self, pos: IVec3) -> Option<Block> {
        let pos = pos + IVec3::ONE;
        let idx = self.pos_idx(pos.as_uvec3());
        if idx >= self.blocks.len() {
            return None;
        }
        self.blocks[idx]
    }

    pub const fn pos_idx(&self, pos: UVec3) -> usize {
        ((self.extended_size.y * self.extended_size.z * pos.x)
            + (self.extended_size.y * pos.z)
            + pos.y) as usize
    }
}
