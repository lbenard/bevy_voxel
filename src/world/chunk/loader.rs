use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use bevy::{
    prelude::*,
    render::texture::ImageSampler,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_spectator::SpectatorSystemSet;
use derive_more::{Add, Debug, Div, From};
use futures_lite::future;
use parking_lot::RwLock;

#[cfg(debug_assertions)]
use crate::debug::stats::Average;

use crate::world::{chunk::ChunkState, Chunk, World};

use super::{
    generator::{
        default_materializator::DefaultMaterializator,
        noise_terrain_generator::NoiseTerrainGenerator, Materializator, TerrainGenerator,
    },
    material::TerrainMaterial,
    mesh::ChunkMesh,
    ChunkCoordinates, ChunkMarker, CHUNK_SIZE,
};

#[derive(Default, Add, Div, From, Copy, Clone, Debug)]
#[debug("{_0:?}")]
struct GenerationDuration(Duration);
#[derive(Default, Add, Div, From, Copy, Clone, Debug)]
#[debug("{_0:?}")]
struct MeshingDuration(Duration);

struct ComputeChunkResult {
    absolute_position: IVec3,
    mesh: Mesh,
    generation_duration: GenerationDuration,
    meshing_duration: MeshingDuration,
}

#[derive(Component)]
pub(self) struct ComputeChunk(Task<ComputeChunkResult>);

pub struct ChunkLoaderPlugin {
    default_load_distance: u32,
    default_unload_distance: u32,
}

impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    Self::load_chunks,
                    Self::handle_chunk_tasks,
                    Self::unload_chunks,
                )
                    .chain()
                    .after(SpectatorSystemSet),
                Self::set_images_to_nearest,
            ),
        )
        .insert_resource(RenderDistance {
            load_distance: self.default_load_distance,
            unload_distance: self.default_unload_distance,
        });

        #[cfg(debug_assertions)]
        app.init_resource::<Average<GenerationDuration>>()
            .init_resource::<Average<MeshingDuration>>()
            .add_systems(
                Update,
                (
                    Average::<GenerationDuration>::egui_debug,
                    Average::<MeshingDuration>::egui_debug,
                ),
            );
    }
}

impl ChunkLoaderPlugin {
    pub fn new(load_distance: u32, unload_distance: u32) -> Self {
        Self {
            default_load_distance: load_distance,
            default_unload_distance: unload_distance,
        }
    }

    fn unload_chunks(
        mut commands: Commands,
        source: Query<(&Transform, &ChunkLoaderSource)>,
        render_distance: Res<RenderDistance>,
        mut world: ResMut<World>,
    ) {
        let unload_distance = render_distance.unload_distance;
        let Ok((source_transform, _)) = source.get_single() else { return };
        let coordinates =
            Self::chunk_coordinates_within_range(source_transform.translation, unload_distance);
        let out_of_range = world
            .chunks
            .extract_if(|k, _v| !coordinates.contains(k))
            .map(|(_k, v)| v)
            .collect::<Vec<Arc<RwLock<Chunk>>>>();
        for chunk in out_of_range {
            let chunk = chunk.read();
            commands.entity(chunk.entity).despawn();
            world.remove_chunk(chunk.coordinates);
        }
    }

    fn load_chunks(
        mut commands: Commands,
        source: Query<(&Transform, &ChunkLoaderSource)>,
        render_distance: Res<RenderDistance>,
        mut world: ResMut<World>,
    ) {
        let thread_pool = AsyncComputeTaskPool::get();

        let load_distance = render_distance.load_distance;
        let Ok((source_transform, _)) = source.get_single() else { return };
        let coordinates =
            Self::chunk_coordinates_within_range(source_transform.translation, load_distance);

        for chunk_coordinates in coordinates {
            if world.get_chunk_mut(chunk_coordinates).is_none() {
                let mut new_chunk =
                    commands.spawn((ChunkMarker, chunk_coordinates, ChunkState::Loading));
                world.spawn_chunk(new_chunk.id(), chunk_coordinates);
                let task = Self::new_chunk_task(thread_pool, chunk_coordinates, &mut world);
                new_chunk.insert(ComputeChunk(task));
            }
        }
    }

    fn handle_chunk_tasks(
        mut commands: Commands,
        mut chunk_tasks: Query<(Entity, &mut ComputeChunk)>,
        mut chunk_states: Query<(Entity, &mut ChunkState)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TerrainMaterial>>,
        #[cfg(debug_assertions)] mut generation_average: ResMut<Average<GenerationDuration>>,
        #[cfg(debug_assertions)] mut meshing_average: ResMut<Average<MeshingDuration>>,
    ) {
        let mut material: TerrainMaterial = Color::rgb(0.1, 0.9, 0.0).into();
        material.metallic = 0.0;
        material.reflectance = 0.0;
        material.perceptual_roughness = 1.0;

        for (entity, mut chunk_task) in &mut chunk_tasks.iter_mut() {
            if let Some(chunk) = future::block_on(future::poll_once(&mut chunk_task.0)) {
                #[cfg(debug_assertions)]
                generation_average.add(chunk.generation_duration);
                #[cfg(debug_assertions)]
                meshing_average.add(chunk.meshing_duration);

                if let Some(mut entity_command) = commands.get_entity(entity) {
                    entity_command.insert((MaterialMeshBundle {
                        mesh: meshes.add(chunk.mesh),
                        material: materials.add(material.clone()),
                        transform: Transform::from_xyz(
                            chunk.absolute_position.x as f32,
                            chunk.absolute_position.y as f32,
                            chunk.absolute_position.z as f32,
                        ),
                        ..default()
                    },));

                    commands.entity(entity).remove::<ComputeChunk>();
                    *chunk_states.get_mut(entity).unwrap().1 = ChunkState::Rendered;
                }
            }
        }
    }

    fn set_images_to_nearest(
        mut ev_asset: EventReader<AssetEvent<Image>>,
        mut assets: ResMut<Assets<Image>>,
    ) {
        for ev in ev_asset.iter() {
            match ev {
                AssetEvent::Created { handle } => {
                    if let Some(image) = assets.get_mut(handle) {
                        image.sampler_descriptor = ImageSampler::nearest();
                    }
                }
                _ => {}
            }
        }
    }

    fn new_chunk_task(
        thread_pool: &AsyncComputeTaskPool,
        chunk_coordinates: ChunkCoordinates,
        world: &mut World,
    ) -> Task<ComputeChunkResult> {
        let chunk = world.get_chunk_mut(chunk_coordinates).unwrap().clone();

        thread_pool.spawn(async move {
            let generation_timer = Instant::now();

            let generator = NoiseTerrainGenerator::new();
            let materializator = DefaultMaterializator {};
            let absolute_position = IVec3::new(
                chunk_coordinates.0.x * CHUNK_SIZE.x as i32,
                chunk_coordinates.0.y * CHUNK_SIZE.y as i32,
                chunk_coordinates.0.z * CHUNK_SIZE.z as i32,
            );

            let chunk_shape = generator.generate(absolute_position, super::Shape {});
            chunk.write().terrain = Some(chunk_shape);
            let grid = materializator.materialize(&chunk.read().terrain.as_ref().unwrap());
            chunk.write().grid = Some(grid);

            let generation_duration = generation_timer.elapsed();

            let meshing_timer = Instant::now();
            let mesh = ChunkMesh::new()
                .mesh_grid(&chunk.read().grid.as_ref().unwrap())
                .mesh();
            let meshing_duration = meshing_timer.elapsed();
            ComputeChunkResult {
                mesh,
                absolute_position,
                generation_duration: generation_duration.into(),
                meshing_duration: meshing_duration.into(),
            }
        })
    }

    fn chunk_coordinates_within_range(source: Vec3, radius: u32) -> Vec<ChunkCoordinates> {
        let mut chunks = Vec::new();
        let source_coordinates = Vec3::new(
            source.x / CHUNK_SIZE.x as f32,
            0.0,
            source.z / CHUNK_SIZE.z as f32,
        );
        let current_chunk = (source / CHUNK_SIZE.as_vec3()).as_ivec3();

        for x in (current_chunk.x - radius as i32)..(current_chunk.x + radius as i32) {
            for z in (current_chunk.z - radius as i32)..(current_chunk.z + radius as i32) {
                let chunk_coordinates = IVec3::new(x, 0, z);
                let chunk_middle = chunk_coordinates.as_vec3() + Vec3::ONE / 2.0;
                let distance_squared = (chunk_middle - source_coordinates).length_squared();

                if distance_squared < (radius * radius) as f32 {
                    chunks.push(ChunkCoordinates(chunk_coordinates));
                }
            }
        }

        chunks
    }
}

#[derive(Component)]
pub struct ChunkLoaderSource;

#[derive(Resource)]
pub struct RenderDistance {
    pub load_distance: u32,
    pub unload_distance: u32,
}
