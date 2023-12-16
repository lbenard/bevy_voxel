use crate::world::voxel::material::Material;
use crate::world::voxel::shape::Volume;
use crate::world::{World, WorldChunk};

use bevy::math::IVec3;
use bevy::prelude::Mesh;
use bevy::utils::HashMap;
use bevy::{
    math::UVec3,
    prelude::Vec3,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use rand::Rng;

use super::material::ATTRIBUTE_VOXEL_ID;
use super::Coordinates;

pub mod voxel;

#[derive(Default)]
pub struct ChunkMesh {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    voxel_ids: Vec<u32>,
    indices: Vec<u32>,
}

pub struct AdjacentChunks {
    north: WorldChunk,
    east: WorldChunk,
    south: WorldChunk,
    west: WorldChunk,
}

impl World {
    pub fn from_adjacent_chunks(chunk: WorldChunk, adjacent_chunks: AdjacentChunks) -> Self {
        Self {
            chunks: HashMap::from([
                (chunk.read().coordinates, chunk.clone()),
                (
                    adjacent_chunks.north.read().coordinates,
                    adjacent_chunks.north.clone(),
                ),
                (
                    adjacent_chunks.east.read().coordinates,
                    adjacent_chunks.east.clone(),
                ),
                (
                    adjacent_chunks.south.read().coordinates,
                    adjacent_chunks.south.clone(),
                ),
                (
                    adjacent_chunks.west.read().coordinates,
                    adjacent_chunks.west.clone(),
                ),
            ]),
        }
    }

    pub fn get_adjacent_chunks(&self, chunk: WorldChunk) -> Result<AdjacentChunks, ()> {
        let base_coordinates = chunk.read().coordinates;

        let north = self.get_chunk(base_coordinates + Coordinates(IVec3::new(0, 0, 1)));
        let east = self.get_chunk(base_coordinates + Coordinates(IVec3::new(1, 0, 0)));
        let south = self.get_chunk(base_coordinates + Coordinates(IVec3::new(0, 0, -1)));
        let west = self.get_chunk(base_coordinates + Coordinates(IVec3::new(-1, 0, 0)));

        if let (Some(north), Some(east), Some(south), Some(west)) = (north, east, south, west) {
            if north.read().terrain.is_some()
                && east.read().terrain.is_some()
                && south.read().terrain.is_some()
                && west.read().terrain.is_some()
            {
                Ok(AdjacentChunks {
                    north,
                    east,
                    south,
                    west,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl ChunkMesh {
    pub fn mesh_chunk(mut self, chunk: WorldChunk, world: &World) -> Self {
        let chunk_lock = chunk.read();
        let terrain = &chunk_lock.terrain.as_ref().unwrap();

        for x in 0..terrain.size.x {
            for z in 0..terrain.size.z {
                for y in 0..terrain.size.y {
                    let pos = UVec3 { x, y, z };
                    let voxel_mesh = terrain.voxel_mesh_at_pos(pos);
                    if let Some(voxel_mesh) = voxel_mesh {
                        if voxel_mesh.voxel.shape.volume == Volume::ZeroSixth {
                            continue;
                        }
                        voxel_mesh.mesh(&mut self, chunk.clone(), world);
                    }
                }
            }
        }
        self
    }

    pub fn mesh(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_attribute(ATTRIBUTE_VOXEL_ID, self.voxel_ids);
        mesh.set_indices(Some(Indices::U32(self.indices)));
        mesh
    }

    pub fn add_vertices_at_pos(
        &mut self,
        pos: UVec3,
        triangles: &Vec<[UVec3; 3]>,
        material: &Material,
    ) {
        // let mut rng = rand::thread_rng();
        // let randomize_offset = Vec3::new(
        //     rng.gen_range(-0.04..0.04),
        //     rng.gen_range(-0.04..0.04),
        //     rng.gen_range(-0.04..0.04),
        // );

        for tri in triangles {
            let tri_vertices = tri
                .iter()
                .map(|vertex| (*vertex + pos).as_vec3())
                .collect::<Vec<Vec3>>();
            let mut tri_vertices_array = tri_vertices
                .iter()
                .map(|vertex| vertex.to_array())
                .collect::<Vec<[f32; 3]>>();
            self.vertices.append(&mut tri_vertices_array);

            let normal = Self::normal(tri_vertices[0], tri_vertices[2], tri_vertices[1]);
            // let normal = (normal + randomize_offset).to_array();
            let normal = (normal).to_array();

            // All three vertices should share the same normal because that's how lowpoly works
            self.normals.append(&mut vec![normal, normal, normal]);
            self.uvs
                .append(&mut vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]);
            self.voxel_ids
                .append(&mut vec![material.id, material.id, material.id]);
            let next_index = match self.indices.last() {
                Some(n) => n + 1,
                None => 0,
            };
            self.indices
                .append(&mut [next_index, next_index + 1, next_index + 2].into());
        }
    }

    pub fn normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        (c - a).cross(b - a).normalize()
    }
}
