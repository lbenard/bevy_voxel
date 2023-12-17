use bevy::prelude::*;
use derive_more::{Add, Debug, Div, From};
use futures_lite::future;
use ndshape::Shape as NdShape;
use std::{intrinsics::unlikely, time::Duration};

#[cfg(feature = "debug")]
use crate::debug::stats::Average;

use self::{
    generator::Grid,
    material::{StandardMaterialExtension, TerrainMaterial},
    tasks::{AsyncPool, ComputePool},
};

use super::{
    voxel::{Voxel, VoxelDescriptor},
    World,
};

pub mod generator;
pub mod loader;
pub mod material;
pub mod mesh;
pub mod tasks;

pub const CHUNK_LENGTH: u32 = 32;
pub const CHUNK_HEIGHT: u32 = 128;
pub const CHUNK_SIZE: UVec3 = UVec3::new(CHUNK_LENGTH, CHUNK_HEIGHT, CHUNK_LENGTH);
pub type Shape = ndshape::ConstShape3u32<CHUNK_LENGTH, CHUNK_HEIGHT, CHUNK_LENGTH>;

#[derive(Default, Add, Div, From, Copy, Clone, Debug)]
#[debug("{_0:?}")]
pub struct GenerationDuration(Duration);
#[derive(Default, Add, Div, From, Copy, Clone, Debug)]
#[debug("{_0:?}")]
pub struct MeshingDuration(Duration);

pub struct Chunk {
    pub entity: Entity,
    pub state: State,
    pub coordinates: Coordinates,
    pub absolute_position: IVec3,
    pub grid: Option<Grid>,
    pub terrain: Option<Terrain>,
    pub dirty: bool,
}

impl Chunk {
    pub fn get_voxel(&self, relative_position: UVec3) -> Option<Voxel> {
        if let Some(terrain) = &self.terrain {
            let Some(voxel_descriptor) = terrain.voxel_at_pos(relative_position.as_ivec3()) else { return None };
            Some(Voxel {
                position: self.absolute_position + relative_position.as_ivec3(),
                shape: voxel_descriptor.shape,
                material: voxel_descriptor.material,
            })
        } else {
            None
        }
    }

    pub fn get_relative_position(&self, position: IVec3) -> UVec3 {
        (position - self.coordinates.0 * CHUNK_SIZE.as_ivec3()).as_uvec3()
    }

    fn handle_generation_tasks(
        mut commands: Commands,
        mut generation_tasks: Query<(Entity, &mut tasks::AsyncGenerateChunk)>,
        world: Res<World>,
        #[cfg(feature = "debug")] mut generation_average: ResMut<Average<GenerationDuration>>,
    ) {
        for (entity, mut generation_task) in &mut generation_tasks.iter_mut() {
            if let Some(generation_task) =
                future::block_on(future::poll_once(&mut generation_task.0))
            {
                let Some(chunk) = world.get_chunk_by_entity(entity) else { continue };
                let mut lock = chunk.write();

                #[cfg(feature = "debug")]
                generation_average.add(generation_task.generation_duration);

                lock.state = State::Generated;
                lock.terrain = Some(generation_task.terrain);

                commands
                    .entity(entity)
                    .remove::<tasks::AsyncGenerateChunk>();
            }
        }
    }

    fn handle_meshing_tasks(
        mut commands: Commands,
        mut compute_meshing_tasks: Query<(Entity, &mut tasks::MeshChunk<ComputePool>)>,
        mut async_meshing_tasks: Query<(Entity, &mut tasks::MeshChunk<AsyncPool>)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TerrainMaterial>>,
        world: Res<World>,
        #[cfg(feature = "debug")] mut meshing_average: ResMut<Average<MeshingDuration>>,
    ) {
        let material = TerrainMaterial {
            base: StandardMaterial {
                metallic: 0.0,
                reflectance: 0.0,
                perceptual_roughness: 1.0,
                ..default()
            },
            extension: StandardMaterialExtension {},
        };

        for (entity, mut meshing_task) in &mut async_meshing_tasks.iter_mut() {
            if let Some(meshing_task) = future::block_on(future::poll_once(&mut meshing_task.0)) {
                let Some(chunk) = world.get_chunk_by_entity(entity) else { continue };
                let mut lock = chunk.write();
                let mut entity = commands.entity(entity);

                #[cfg(feature = "debug")]
                meshing_average.add(meshing_task.meshing_duration);

                lock.state = State::Meshed;
                lock.dirty = false;
                entity.insert((MaterialMeshBundle {
                    mesh: meshes.add(meshing_task.mesh),
                    material: materials.add(material.clone()),
                    transform: Transform::from_xyz(
                        meshing_task.absolute_position.x as f32,
                        meshing_task.absolute_position.y as f32,
                        meshing_task.absolute_position.z as f32,
                    ),
                    ..default()
                },));
                entity.remove::<tasks::MeshChunk<AsyncPool>>();
            }
        }
        for (entity, mut meshing_task) in &mut compute_meshing_tasks.iter_mut() {
            if let Some(meshing_task) = future::block_on(future::poll_once(&mut meshing_task.0)) {
                let Some(chunk) = world.get_chunk_by_entity(entity) else { continue };
                let mut lock = chunk.write();
                let mut entity = commands.entity(entity);

                #[cfg(feature = "debug")]
                meshing_average.add(meshing_task.meshing_duration);

                lock.state = State::Meshed;
                lock.dirty = false;
                entity.insert((MaterialMeshBundle {
                    mesh: meshes.add(meshing_task.mesh),
                    material: materials.add(material.clone()),
                    transform: Transform::from_xyz(
                        meshing_task.absolute_position.x as f32,
                        meshing_task.absolute_position.y as f32,
                        meshing_task.absolute_position.z as f32,
                    ),
                    ..default()
                },));
                entity.remove::<tasks::MeshChunk<ComputePool>>();
            }
        }
    }
}

#[derive(Component)]
pub struct Marker;

#[derive(Component, PartialEq, Clone, Copy, Eq, Hash, Add, Debug)]
pub struct Coordinates(pub IVec3);

/// Describe the chunk loading state.
/// A chunk is `Loading` at creation, `Loaded` when generated but not displayed, and `Rendered` when generated and displayed.
/// Any `Unloaded` chunk will get deleted from memory.
#[derive(Component, PartialEq, Eq)]
pub enum State {
    Spawned,
    Generated,
    Meshed,
}

pub type VoxelIndex = u8;

pub struct Terrain {
    pub size: UVec3,
    pub voxels: Vec<Option<VoxelDescriptor>>,
    shape: ndshape::ConstShape3u32<CHUNK_LENGTH, CHUNK_HEIGHT, CHUNK_LENGTH>,
}

impl Terrain {
    pub fn voxel_at_pos(&self, pos: IVec3) -> &Option<VoxelDescriptor> {
        if unlikely(pos.cmplt(IVec3::ZERO).any() || pos.cmpge(CHUNK_SIZE.as_ivec3()).any()) {
            return &None;
        }
        self.voxels
            .get(self.shape.linearize(pos.as_uvec3().to_array()) as usize)
            .unwrap()
    }

    pub fn voxel_at_pos_mut(&mut self, pos: IVec3) -> &mut Option<VoxelDescriptor> {
        self.voxels
            .get_mut(self.shape.linearize(pos.as_uvec3().to_array()) as usize)
            .unwrap()
    }
}
