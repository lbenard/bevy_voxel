use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

use crate::chunk::{
    generators::noise_terrain::NoiseTerrain, Chunk, ChunkCoordinates, ChunkState, Grid, CHUNK_SIZE,
};

use super::chunk_mesher::Mesher;

struct ComputeChunkResult {
    absolute_position: IVec2,
    mesh: Mesh,
}

#[derive(Component)]
pub(self) struct ComputeChunk(Task<ComputeChunkResult>);

pub struct ChunkLoaderPlugin {
    render_distance: u32,
}

impl ChunkLoaderPlugin {
    pub fn new(render_distance: u32) -> Self {
        Self { render_distance }
    }

    fn load_chunks(
        mut commands: Commands,
        source: Query<(&Transform, &ChunkLoaderSource)>,
        chunks: Query<(Entity, &Chunk, &ChunkCoordinates, &mut ChunkState)>,
        render_distance: Res<RenderDistance>,
    ) {
        let thread_pool = AsyncComputeTaskPool::get();

        let render_distance = render_distance.render_distance as i32;
        let Ok((source_transform, _)) = source.get_single() else { return };
        let source_coordinates = Vec2::new(
            source_transform.translation.x / CHUNK_SIZE.x as f32,
            source_transform.translation.z / CHUNK_SIZE.z as f32,
        );
        let current_chunk = (source_transform.translation / CHUNK_SIZE.as_vec3()).as_ivec3();

        for x in (current_chunk.x - render_distance)..(current_chunk.x + render_distance) {
            for z in (current_chunk.z - render_distance)..(current_chunk.z + render_distance) {
                let chunk_coordinates = IVec2::new(x, z);
                let chunk_middle = chunk_coordinates.as_vec2() + Vec2::ONE / 2.0;
                let distance_squared = (chunk_middle - source_coordinates).length_squared();

                if distance_squared < (render_distance * render_distance) as f32 {
                    let chunk = Self::get_chunk(&chunks, ChunkCoordinates(chunk_coordinates));
                    if let Some(_entity) = chunk {
                        // TODO: unload those fuckers
                        // if *state != ChunkState::Loaded {
                        //     commands.entity(entity).insert(ChunkState::Loaded);
                        // }
                    } else {
                        info!("Generating chunk at {x} {z}");
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
    }

    fn handle_chunk_tasks(
        mut commands: Commands,
        mut chunk_tasks: Query<(Entity, &mut ComputeChunk)>,
        mut chunk_states: Query<(Entity, &mut ChunkState)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut material: StandardMaterial = Color::rgb(0.0, 0.6, 0.1).into();
        material.metallic = 0.0;
        material.reflectance = 0.0;
        material.perceptual_roughness = 1.0;

        for (entity, mut chunk_task) in &mut chunk_tasks.iter_mut() {
            if let Some(chunk) = future::block_on(future::poll_once(&mut chunk_task.0)) {
                info!(
                    "Spawning chunk at {} {}",
                    chunk.absolute_position.x, chunk.absolute_position.y
                );
                commands.entity(entity).insert(PbrBundle {
                    mesh: meshes.add(chunk.mesh),
                    material: materials.add(material.clone()),
                    transform: Transform::from_xyz(
                        chunk.absolute_position.x as f32,
                        0.0,
                        chunk.absolute_position.y as f32,
                    ),
                    ..default()
                });

                commands.entity(entity).remove::<ComputeChunk>();
                *chunk_states.get_mut(entity).unwrap().1 = ChunkState::Rendered;
            }
        }
    }

    fn get_chunk(
        query: &Query<(Entity, &Chunk, &ChunkCoordinates, &mut ChunkState)>,
        position: ChunkCoordinates,
    ) -> Option<Entity> {
        query
            .iter()
            .find(|(_, ref _chunk, ref chunk_position, ref _chunk_state)| {
                **chunk_position == position
            })
            .map(|(entity, ref _chunk, ref _chunk_position, ref _chunk_state)| entity)
    }

    fn new_chunk_task(
        thread_pool: &AsyncComputeTaskPool,
        chunk_coordinates: IVec2,
    ) -> Task<ComputeChunkResult> {
        thread_pool.spawn(async move {
            let generator = NoiseTerrain::new();
            let absolute_position = IVec2::new(
                chunk_coordinates.x * CHUNK_SIZE.x as i32,
                chunk_coordinates.y * CHUNK_SIZE.z as i32,
            );
            let grid = Grid::new(CHUNK_SIZE).generate(
                IVec3::new(absolute_position.x, 0, absolute_position.y),
                &generator,
            );

            let mesh = Mesher::new().mesh_grid(&grid).mesh();
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
            .insert_resource(RenderDistance {
                render_distance: self.render_distance,
            });
    }
}

#[derive(Component)]
pub struct ChunkLoaderSource;

#[derive(Resource)]
pub struct RenderDistance {
    pub render_distance: u32,
}
