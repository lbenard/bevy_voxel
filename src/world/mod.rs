use bevy::{
    prelude::{Entity, IVec3, Plugin, Resource},
    utils::HashMap,
};
use parking_lot::RwLock;
use std::sync::Arc;

use self::{
    chunk::{Chunk, CHUNK_SIZE},
    voxel::Voxel,
};

pub mod chunk;
// pub mod raycast;
pub mod voxel;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<World>();
    }
}

pub type WorldChunk = Arc<RwLock<Chunk>>;

#[derive(Resource, Default)]
pub struct World {
    pub chunks: HashMap<chunk::Coordinates, WorldChunk>,
}

impl From<Chunk> for WorldChunk {
    fn from(val: Chunk) -> Self {
        Arc::new(RwLock::new(val))
    }
}

impl World {
    pub fn spawn_chunk(&mut self, entity: Entity, coordinates: chunk::Coordinates) {
        self.chunks.insert(
            coordinates,
            Chunk {
                entity,
                state: chunk::State::Spawned,
                coordinates,
                absolute_position: coordinates.0 * CHUNK_SIZE.as_ivec3(),
                grid: None,
                terrain: None,
            }
            .into(),
        );
    }

    pub fn remove_chunk(&mut self, coordinates: chunk::Coordinates) {
        self.chunks.remove(&coordinates);
    }

    pub fn get_chunk(&self, coordinates: chunk::Coordinates) -> Option<WorldChunk> {
        self.chunks.get(&coordinates).map(|chunk| chunk.clone())
    }

    pub fn get_chunk_by_entity(&self, entity: Entity) -> Option<WorldChunk> {
        self.chunks
            .values()
            .find(|chunk| chunk.read().entity == entity)
            .map(|chunk| chunk.clone())
    }

    pub fn get_voxel(&self, position: IVec3) -> Option<Voxel> {
        let chunk_coordinates = chunk::Coordinates(
            (position.as_vec3() / CHUNK_SIZE.as_vec3())
                .floor()
                .as_ivec3(),
        );
        let chunk = self.get_chunk(chunk_coordinates)?;
        let relative_position = (position - chunk_coordinates.0 * CHUNK_SIZE.as_ivec3()).as_uvec3();
        let voxel = chunk.read().get_voxel(relative_position)?;
        Some(voxel)
    }
}
