use bevy::prelude::{Entity, IVec3, Query, Resource};

use crate::chunk::{Chunk, ChunkCoordinates, CHUNK_SIZE};

use self::block::Block;

pub mod block;
pub mod raycast;
pub mod terrain;

#[derive(Resource, Default)]
pub struct World {
    pub chunks: Vec<Chunk>,
}

impl World {
    pub fn get_chunk(&self, coordinates: ChunkCoordinates) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| chunk.coordinates == coordinates)
    }

    pub fn get_block(&self, position: IVec3) -> Option<Block> {
        let chunk_coordinates = ChunkCoordinates(position / CHUNK_SIZE.as_ivec3());
        let chunk = self.get_chunk(chunk_coordinates)?;
        let relative_position = (position - chunk_coordinates.0 * CHUNK_SIZE.as_ivec3()).as_uvec3();
        let block = chunk.get_block(relative_position)?;
        Some(block)
    }
}
