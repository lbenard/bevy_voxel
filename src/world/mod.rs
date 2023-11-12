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

#[derive(Resource, Default)]
pub struct World {
    pub chunks: HashMap<chunk::Coordinates, Arc<RwLock<Chunk>>>,
}

impl From<Chunk> for Arc<RwLock<Chunk>> {
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
                coordinates,
                absolute_position: coordinates.0 * CHUNK_SIZE.as_ivec3(),
                grid: None,
                terrain: None,
            }
            .into(),
        );
    }

    #[allow(dead_code)]
    pub fn remove_chunk(&mut self, coordinates: chunk::Coordinates) {
        self.chunks.remove(&coordinates);
    }

    #[allow(dead_code)]
    pub fn get_chunk(&self, coordinates: chunk::Coordinates) -> Option<&Arc<RwLock<Chunk>>> {
        self.chunks.get(&coordinates)
    }

    #[allow(dead_code)]
    pub fn get_chunk_mut(
        &mut self,
        coordinates: chunk::Coordinates,
    ) -> Option<&mut Arc<RwLock<Chunk>>> {
        self.chunks.get_mut(&coordinates)
    }

    #[allow(dead_code)]
    pub fn get_voxel(self, position: IVec3) -> Option<Voxel> {
        let chunk_coordinates = chunk::Coordinates(position / CHUNK_SIZE.as_ivec3());
        let chunk = self.get_chunk(chunk_coordinates)?.read();
        let relative_position = (position - chunk_coordinates.0 * CHUNK_SIZE.as_ivec3()).as_uvec3();
        let voxel = chunk.get_voxel(relative_position)?;
        Some(voxel)
    }
}
