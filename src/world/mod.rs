use bevy::{
    prelude::{Entity, IVec3, Plugin, Resource, UVec3},
    utils::HashMap,
};

use self::{
    chunk::{generator::Terrain, ChunkCoordinates, Grid, CHUNK_SIZE},
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
    pub chunks: HashMap<ChunkCoordinates, Chunk>,
}

impl World {
    pub fn spawn_chunk(&mut self, entity: Entity, coordinates: ChunkCoordinates) {
        self.chunks.insert(
            coordinates,
            Chunk {
                entity,
                coordinates,
                absolute_position: coordinates.0 * CHUNK_SIZE.as_ivec3(),
                terrain: None,
                grid: None,
            },
        );
    }

    #[allow(dead_code)]
    pub fn remove_chunk(&mut self, coordinates: ChunkCoordinates) {
        self.chunks.remove(&coordinates);
    }

    #[allow(dead_code)]
    pub fn get_chunk(&self, coordinates: ChunkCoordinates) -> Option<&Chunk> {
        self.chunks.get(&coordinates)
    }

    #[allow(dead_code)]
    pub fn get_chunk_mut(&mut self, coordinates: ChunkCoordinates) -> Option<&mut Chunk> {
        self.chunks.get_mut(&coordinates)
    }

    #[allow(dead_code)]
    pub fn get_voxel(self, position: IVec3) -> Option<Voxel> {
        let chunk_coordinates = ChunkCoordinates(position / CHUNK_SIZE.as_ivec3());
        let chunk = self.get_chunk(chunk_coordinates)?;
        let relative_position = (position - chunk_coordinates.0 * CHUNK_SIZE.as_ivec3()).as_uvec3();
        let voxel = chunk.get_voxel(relative_position)?;
        Some(voxel)
    }
}

pub struct Chunk {
    entity: Entity,
    coordinates: ChunkCoordinates,
    absolute_position: IVec3,
    pub terrain: Option<Terrain>,
    pub grid: Option<Grid>, // to replace, grid has what's needed for meshing
}

impl Chunk {
    #[allow(dead_code)]
    pub fn get_voxel(&self, relative_position: UVec3) -> Option<Voxel> {
        if let Some(grid) = &self.grid {
            let voxel_descriptor = grid.voxel_at_pos(relative_position.as_ivec3())?;
            Some(Voxel {
                position: self.absolute_position + relative_position.as_ivec3(),
                shape: voxel_descriptor.shape,
                material: voxel_descriptor.material,
            })
        } else {
            None
        }
    }
}
