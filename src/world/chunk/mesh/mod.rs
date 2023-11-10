use crate::world::voxel::material::Material;
use crate::world::voxel::shape::Volume;

use bevy::prelude::Mesh;
use bevy::{
    math::UVec3,
    prelude::Vec3,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use rand::Rng;

use super::material::ATTRIBUTE_VOXEL_ID;
use super::Terrain;

pub mod voxel;

pub struct ChunkMesh {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    voxel_ids: Vec<u32>,
    indices: Vec<u32>,
}

impl ChunkMesh {
    pub fn new() -> Self {
        Self {
            // TODO: allocate with capacity to avoid obvious vector re-allocations
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            voxel_ids: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn mesh_terrain(mut self, terrain: &Terrain) -> Self {
        for x in 0..terrain.size.x {
            for z in 0..terrain.size.z {
                for y in 0..terrain.size.y {
                    let pos = UVec3 { x, y: y as u32, z };
                    let voxel_mesh = terrain.voxel_mesh_at_pos(pos);
                    if let Some(voxel_mesh) = voxel_mesh {
                        if voxel_mesh.voxel.shape.volume == Volume::ZeroSixth {
                            continue;
                        }
                        voxel_mesh.mesh(&mut self, &terrain);
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
        let mut rng = rand::thread_rng();
        let randomize_offset = Vec3::new(
            rng.gen_range(-0.04..0.04),
            rng.gen_range(-0.04..0.04),
            rng.gen_range(-0.04..0.04),
        );

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
            let normal = (normal + randomize_offset).to_array();

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
