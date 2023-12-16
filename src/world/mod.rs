use bevy::{prelude::*, utils::HashMap};
use parking_lot::RwLock;
use std::sync::Arc;

use self::{
    chunk::{Chunk, CHUNK_SIZE},
    voxel::{Voxel, VoxelDescriptor},
};

pub mod chunk;
pub mod raycast;
pub mod voxel;

// World systems are divided into two consecutive sets:
// - The simulation set where every terrain update happens
// - The tasks set which queues and handle finished tasks
// Those two are separated so that the game properly waits for every update to have finished before meshing anything.
// This also helps avoiding any task to be called twice during a single frame.
#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub struct WorldSimulationSystemSet;
#[derive(SystemSet, Hash, Debug, Clone, Eq, PartialEq)]
pub struct WorldTasksSystemSet;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<World>()
            .configure_sets(Update, WorldTasksSystemSet.after(WorldSimulationSystemSet));
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
                dirty: false,
            }
            .into(),
        );
    }

    pub fn remove_chunk(&mut self, coordinates: chunk::Coordinates) {
        self.chunks.remove(&coordinates);
    }

    pub fn get_chunk(&self, coordinates: chunk::Coordinates) -> Option<WorldChunk> {
        self.chunks.get(&coordinates).cloned()
    }

    pub fn get_chunk_at_pos(&self, position: IVec3) -> Option<WorldChunk> {
        self.get_chunk(Self::position_to_chunk_coordinates(position))
    }

    pub fn get_chunk_by_entity(&self, entity: Entity) -> Option<WorldChunk> {
        self.chunks
            .values()
            .find(|chunk| chunk.read().entity == entity)
            .cloned()
    }

    pub fn get_voxel(&self, position: IVec3) -> Option<Voxel> {
        let chunk_coordinates = Self::position_to_chunk_coordinates(position);
        let chunk = self.get_chunk(chunk_coordinates)?;
        let relative_position = (position - chunk_coordinates.0 * CHUNK_SIZE.as_ivec3()).as_uvec3();
        let voxel = chunk.read().get_voxel(relative_position)?;
        Some(voxel)
    }

    pub fn position_to_chunk_coordinates(position: IVec3) -> chunk::Coordinates {
        chunk::Coordinates(
            (position.as_vec3() / CHUNK_SIZE.as_vec3())
                .floor()
                .as_ivec3(),
        )
    }
}
