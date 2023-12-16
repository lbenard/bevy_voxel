use bevy::prelude::*;

#[cfg(feature = "debug")]
use super::{GenerationDuration, MeshingDuration};
#[cfg(feature = "debug")]
use crate::debug::stats::Average;

use crate::world::{
    chunk, Chunk, World, WorldChunk, WorldSimulationSystemSet, WorldTasksSystemSet,
};

use super::{
    tasks::{self, AsyncPool, ComputePool},
    State, CHUNK_LENGTH, CHUNK_SIZE,
};

pub struct ChunkLoaderPlugin {
    default_load_distance: u32,
    default_unload_distance: u32,
}

impl Plugin for ChunkLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (Self::load_chunks, Self::unload_chunks, Self::mesh_chunks)
                    .chain()
                    .in_set(WorldSimulationSystemSet),
                (Self::mesh_dirty_chunks)
                    .after(WorldSimulationSystemSet)
                    .before(WorldTasksSystemSet),
                (Chunk::handle_generation_tasks, Chunk::handle_meshing_tasks)
                    .in_set(WorldTasksSystemSet),
            ),
        )
        .insert_resource(RenderDistance {
            load_distance: self.default_load_distance,
            unload_distance: self.default_unload_distance,
        });

        #[cfg(feature = "debug")]
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

    fn load_chunks(
        mut commands: Commands,
        source: Query<(&Transform, &ChunkLoaderSource)>,
        render_distance: Res<RenderDistance>,
        mut world: ResMut<World>,
    ) {
        let load_distance = render_distance.load_distance / CHUNK_LENGTH;
        let Ok((source_transform, _)) = source.get_single() else { return };
        let mut coordinates =
            Self::chunk_coordinates_within_range(source_transform.translation, load_distance);

        // TODO: might do absolutely nothing
        coordinates.sort_by(|a, b| {
            let a: Vec3 = a.0.as_vec3();
            let b: Vec3 = b.0.as_vec3();
            let source = source_transform.translation;
            source.distance(a).partial_cmp(&source.distance(b)).unwrap()
        });

        for chunk_coordinates in coordinates {
            if world.get_chunk(chunk_coordinates).is_none() {
                let mut chunk_entity = commands.spawn((chunk::Marker,));
                world.spawn_chunk(chunk_entity.id(), chunk_coordinates);
                let chunk = world.get_chunk(chunk_coordinates).unwrap().clone();
                let task = chunk::tasks::new_generate_chunk_task(chunk, chunk_coordinates);
                chunk_entity.insert(chunk::tasks::AsyncGenerateChunk(task));
            }
        }
    }

    fn mesh_chunks(
        mut commands: Commands,
        queued_chunks: Query<(Entity, With<tasks::MeshChunk<AsyncPool>>)>,
        world: Res<crate::world::World>,
    ) {
        let queued_chunks_entities = queued_chunks.iter().map(|c| c.0).collect::<Vec<Entity>>();
        let generated_chunks = world
            .chunks
            .values()
            .filter(|chunk| chunk.read().state == State::Generated);
        let chunks_to_mesh = generated_chunks
            .filter(|chunk| !queued_chunks_entities.contains(&chunk.read().entity))
            .collect::<Vec<&WorldChunk>>();

        for chunk in chunks_to_mesh {
            let Ok(adjacent_chunks) = world.get_adjacent_chunks(chunk.clone()) else { continue };

            let task = tasks::new_mesh_chunk_task::<AsyncPool>(
                chunk.clone(),
                adjacent_chunks,
                chunk.read().coordinates,
            );
            commands
                .entity(chunk.read().entity)
                .insert(tasks::MeshChunk::<AsyncPool>::from_task(task));
        }
    }

    fn unload_chunks(
        mut commands: Commands,
        source: Query<(&Transform, &ChunkLoaderSource)>,
        render_distance: Res<RenderDistance>,
        mut world: ResMut<World>,
    ) {
        let unload_distance = render_distance.unload_distance / CHUNK_LENGTH;
        let Ok((source_transform, _)) = source.get_single() else { return };
        let coordinates =
            Self::chunk_coordinates_within_range(source_transform.translation, unload_distance);
        let out_of_range = world
            .chunks
            .extract_if(|k, _v| !coordinates.contains(k))
            .map(|(_k, v)| v)
            .collect::<Vec<WorldChunk>>();
        for chunk in out_of_range {
            let chunk = chunk.read();
            commands.entity(chunk.entity).despawn();
            world.remove_chunk(chunk.coordinates);
        }
    }

    fn mesh_dirty_chunks(mut commands: Commands, world: Res<crate::world::World>) {
        let dirty_chunks = world
            .chunks
            .values()
            .filter(|chunk| chunk.read().dirty)
            .collect::<Vec<&WorldChunk>>();

        for chunk in dirty_chunks {
            chunk.write().dirty = false;

            let Ok(adjacent_chunks) = world.get_adjacent_chunks(chunk.clone()) else { continue };
            let task = tasks::new_mesh_chunk_task::<AsyncPool>(
                chunk.clone(),
                adjacent_chunks,
                chunk.read().coordinates,
            );
            commands
                .entity(chunk.read().entity)
                .insert(tasks::MeshChunk::<AsyncPool>::from_task(task));
        }
    }

    fn chunk_coordinates_within_range(source: Vec3, radius: u32) -> Vec<chunk::Coordinates> {
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
                    chunks.push(chunk::Coordinates(chunk_coordinates));
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
