use bevy::prelude::*;
use derive_more::{Add, Debug, Div, From};
use futures_lite::future;
use ndshape::Shape as NdShape;
use std::{intrinsics::unlikely, time::Duration};

#[cfg(debug_assertions)]
use crate::debug::stats::Average;

use self::{
    generator::Grid,
    material::{StandardMaterialExtension, TerrainMaterial},
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
}

impl Chunk {
    #[allow(dead_code)]
    pub fn get_voxel(&self, relative_position: UVec3) -> Option<Voxel> {
        if let Some(terrain) = &self.terrain {
            let voxel_descriptor = terrain.voxel_at_pos(relative_position.as_ivec3())?;
            Some(Voxel {
                position: self.absolute_position + relative_position.as_ivec3(),
                shape: voxel_descriptor.shape,
                material: voxel_descriptor.material,
            })
        } else {
            None
        }
    }

    fn handle_generation_tasks(
        mut commands: Commands,
        mut generation_tasks: Query<(Entity, &mut tasks::GenerateChunk)>,
        world: Res<World>,
        #[cfg(debug_assertions)] mut generation_average: ResMut<Average<GenerationDuration>>,
    ) {
        for (entity, mut generation_task) in &mut generation_tasks.iter_mut() {
            if let Some(generation_task) =
                future::block_on(future::poll_once(&mut generation_task.0))
            {
                let chunk = world.get_chunk_by_entity(entity).unwrap();

                #[cfg(debug_assertions)]
                generation_average.add(generation_task.generation_duration);

                chunk.write().state = State::Generated;
                chunk.write().terrain = Some(generation_task.terrain);

                commands.entity(entity).remove::<tasks::GenerateChunk>();
            }
        }
    }

    fn handle_meshing_tasks(
        mut commands: Commands,
        mut meshing_tasks: Query<(Entity, &mut tasks::MeshChunk)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TerrainMaterial>>,
        world: Res<World>,
        #[cfg(debug_assertions)] mut meshing_average: ResMut<Average<MeshingDuration>>,
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

        for (entity, mut meshing_task) in &mut meshing_tasks.iter_mut() {
            if let Some(meshing_task) = future::block_on(future::poll_once(&mut meshing_task.0)) {
                let chunk = world.get_chunk_by_entity(entity).unwrap();
                let mut entity = commands.entity(entity);

                #[cfg(debug_assertions)]
                meshing_average.add(meshing_task.meshing_duration);

                chunk.write().state = State::Meshed;
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
                entity.remove::<tasks::MeshChunk>();
            }
        }
    }
}

#[derive(Component)]
pub struct Marker;

#[derive(Component, PartialEq, Clone, Copy, Eq, Hash)]
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
    pub fn voxel_at_pos(&self, pos: IVec3) -> Option<VoxelDescriptor> {
        if unlikely(pos.cmplt(IVec3::ZERO).any() || pos.cmpge(CHUNK_SIZE.as_ivec3()).any()) {
            return None;
        }
        self.voxels[self.shape.linearize(pos.as_uvec3().to_array()) as usize]
    }
}
