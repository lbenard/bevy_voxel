use bevy::{
    prelude::*,
    render::texture::ImageSampler,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_mod_raycast::RaycastMesh;
use futures_lite::future;

use crate::{
    chunk::{
        generators::noise_terrain::NoiseTerrain, mesh::ChunkMesh, Chunk, ChunkCoordinates,
        ChunkState, Grid, CHUNK_SIZE,
    },
    terrain::TerrainRaycastSet,
};

use super::material::TerrainMaterial;

struct ComputeChunkResult {
    absolute_position: IVec3,
    mesh: Mesh,
}

#[derive(Component)]
pub(self) struct ComputeChunk(Task<ComputeChunkResult>);

pub struct ChunkLoaderPlugin {
    load_distance: u32,
    unload_distance: u32,
}

impl ChunkLoaderPlugin {
    pub fn new(load_distance: u32, unload_distance: u32) -> Self {
        Self {
            load_distance,
            unload_distance,
        }
    }

    fn load_chunks(
        mut commands: Commands,
        source: Query<(&Transform, &ChunkLoaderSource)>,
        chunks: Query<(Entity, &Chunk, &ChunkCoordinates)>,
        render_distance: Res<RenderDistance>,
    ) {
        let thread_pool = AsyncComputeTaskPool::get();

        let load_distance = render_distance.load_distance as i32;
        let Ok((source_transform, _)) = source.get_single() else { return };
        let source_coordinates = Vec3::new(
            source_transform.translation.x / CHUNK_SIZE.x as f32,
            0.0,
            source_transform.translation.z / CHUNK_SIZE.z as f32,
        );
        let current_chunk = (source_transform.translation / CHUNK_SIZE.as_vec3()).as_ivec3();

        // Load chunks
        for x in (current_chunk.x - load_distance)..(current_chunk.x + load_distance) {
            for z in (current_chunk.z - load_distance)..(current_chunk.z + load_distance) {
                let chunk_coordinates = IVec3::new(x, 0, z);
                let chunk_middle = chunk_coordinates.as_vec3() + Vec3::ONE / 2.0;
                let distance_squared = (chunk_middle - source_coordinates).length_squared();

                if distance_squared < (load_distance * load_distance) as f32 {
                    let chunk = Self::get_chunk(&chunks, ChunkCoordinates(chunk_coordinates));
                    if chunk.is_none() {
                        info!(
                            "Generating chunk at {} {}",
                            chunk_coordinates.x, chunk_coordinates.z
                        );
                        let task = Self::new_chunk_task(thread_pool, chunk_coordinates);
                        commands.spawn((
                            Chunk,
                            ChunkCoordinates(chunk_coordinates),
                            ChunkState::Loading,
                            ComputeChunk(task),
                        ));
                    }
                }
            }
        }

        // Unload chunks
        for (entity, _, ChunkCoordinates(chunk_coordinates)) in &mut chunks.iter() {
            let chunk_middle = chunk_coordinates.as_vec3() + Vec3::ONE / 2.0;
            let distance_squared = (chunk_middle - source_coordinates).length_squared();

            if distance_squared
                > (render_distance.unload_distance * render_distance.unload_distance) as f32
            {
                info!(
                    "Unloading chunk at {} {}",
                    chunk_coordinates.x, chunk_coordinates.z
                );
                commands.entity(entity).despawn();
            }
        }
    }

    fn handle_chunk_tasks(
        mut commands: Commands,
        mut chunk_tasks: Query<(Entity, &mut ComputeChunk)>,
        mut chunk_states: Query<(Entity, &mut ChunkState)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TerrainMaterial>>,
    ) {
        let mut material: TerrainMaterial = Color::rgb(1.0, 1.0, 1.0).into();
        material.metallic = 0.0;
        material.reflectance = 0.0;
        material.perceptual_roughness = 1.0;

        for (entity, mut chunk_task) in &mut chunk_tasks.iter_mut() {
            if let Some(chunk) = future::block_on(future::poll_once(&mut chunk_task.0)) {
                info!(
                    "Spawning chunk at {} {}",
                    chunk.absolute_position.x, chunk.absolute_position.y
                );
                if let Some(mut entity_command) = commands.get_entity(entity) {
                    entity_command.insert((
                        MaterialMeshBundle {
                            mesh: meshes.add(chunk.mesh),
                            material: materials.add(material.clone()),
                            transform: Transform::from_xyz(
                                chunk.absolute_position.x as f32,
                                chunk.absolute_position.y as f32,
                                chunk.absolute_position.z as f32,
                            ),
                            ..default()
                        },
                        RaycastMesh::<TerrainRaycastSet>::default(),
                    ));

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

    fn get_chunk(
        query: &Query<(Entity, &Chunk, &ChunkCoordinates)>,
        position: ChunkCoordinates,
    ) -> Option<Entity> {
        query
            .iter()
            .find(|(_, ref _chunk, ref chunk_position)| **chunk_position == position)
            .map(|(entity, ref _chunk, ref _chunk_position)| entity)
    }

    fn new_chunk_task(
        thread_pool: &AsyncComputeTaskPool,
        chunk_coordinates: IVec3,
    ) -> Task<ComputeChunkResult> {
        thread_pool.spawn(async move {
            let generator = NoiseTerrain::new();
            let absolute_position = IVec3::new(
                chunk_coordinates.x * CHUNK_SIZE.x as i32,
                chunk_coordinates.y * CHUNK_SIZE.y as i32,
                chunk_coordinates.z * CHUNK_SIZE.z as i32,
            );
            let grid = Grid::new(CHUNK_SIZE).generate(
                IVec3::new(
                    absolute_position.x,
                    absolute_position.y,
                    absolute_position.z,
                ),
                &generator,
            );

            let mesh = ChunkMesh::new().mesh_grid(&grid).mesh();
            ComputeChunkResult {
                mesh,
                absolute_position,
            }
        })
    }
}

impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::load_chunks)
            .add_system(Self::handle_chunk_tasks)
            .add_system(Self::set_images_to_nearest)
            .insert_resource(RenderDistance {
                load_distance: self.load_distance,
                unload_distance: self.unload_distance,
            });
    }
}

#[derive(Component)]
pub struct ChunkLoaderSource;

#[derive(Resource)]
pub struct RenderDistance {
    pub load_distance: u32,
    pub unload_distance: u32,
}
