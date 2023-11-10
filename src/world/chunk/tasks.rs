use std::{sync::Arc, time::Instant};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use parking_lot::RwLock;

use super::{
    generator::{
        default_materializator::DefaultMaterializator,
        noise_terrain_generator::NoiseTerrainGenerator, Materializator, TerrainGenerator,
    },
    mesh::ChunkMesh,
    GenerationDuration, MeshingDuration, CHUNK_SIZE,
};
use crate::world::chunk;

pub struct GenerateChunkResult {
    pub chunk: Arc<RwLock<chunk::Chunk>>,
    pub terrain: chunk::Terrain,
    pub generation_duration: GenerationDuration,
}

#[derive(Component)]
pub struct GenerateChunk(pub Task<GenerateChunkResult>);

pub struct MeshChunkResult {
    pub absolute_position: IVec3,
    pub mesh: Mesh,
    pub meshing_duration: MeshingDuration,
}

#[derive(Component)]
pub struct MeshChunk(pub Task<MeshChunkResult>);

pub fn new_generate_chunk_task(
    chunk: Arc<RwLock<chunk::Chunk>>,
    chunk_coordinates: chunk::Coordinates,
) -> Task<GenerateChunkResult> {
    let thread_pool = AsyncComputeTaskPool::get();

    thread_pool.spawn(async move {
        let generation_timer = Instant::now();

        let absolute_position = IVec3::new(
            chunk_coordinates.0.x * CHUNK_SIZE.x as i32,
            chunk_coordinates.0.y * CHUNK_SIZE.y as i32,
            chunk_coordinates.0.z * CHUNK_SIZE.z as i32,
        );
        let generator = NoiseTerrainGenerator::new(absolute_position);
        let materializator = DefaultMaterializator {};

        let grid = generator.generate(crate::world::chunk::Shape {});
        let terrain = materializator.materialize(&grid);

        let generation_duration = generation_timer.elapsed();
        GenerateChunkResult {
            chunk,
            terrain,
            generation_duration: generation_duration.into(),
        }
    })
}

pub fn new_mesh_chunk_task(
    chunk: Arc<RwLock<chunk::Chunk>>,
    chunk_coordinates: chunk::Coordinates,
) -> Task<MeshChunkResult> {
    let thread_pool = AsyncComputeTaskPool::get();

    thread_pool.spawn(async move {
        let absolute_position = IVec3::new(
            chunk_coordinates.0.x * CHUNK_SIZE.x as i32,
            chunk_coordinates.0.y * CHUNK_SIZE.y as i32,
            chunk_coordinates.0.z * CHUNK_SIZE.z as i32,
        );

        let meshing_timer = Instant::now();
        let mesh = ChunkMesh::new()
            .mesh_terrain(&chunk.read().terrain.as_ref().unwrap())
            .mesh();
        let meshing_duration = meshing_timer.elapsed();
        MeshChunkResult {
            mesh,
            absolute_position,
            meshing_duration: meshing_duration.into(),
        }
    })
}
