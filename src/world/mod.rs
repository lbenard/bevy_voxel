use bevy::prelude::{IVec3, Plugin, Resource};

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
    pub fn spawn_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_chunk(&mut self, coordinates: ChunkCoordinates) {
        self.chunks.retain(|chunk| chunk.coordinates != coordinates);
    }

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

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<World>();
    }
}
