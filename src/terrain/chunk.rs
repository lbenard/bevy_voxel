use std::convert::TryInto;
use std::mem::transmute;

use crate::terrain::block::Shape;

use super::{
    block::{SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP, SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP, SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP, Rotation, Volume},
    generator::ChunkGenerator,
    marching_cube::{
        BLOCK_INDEX_TO_SHAPE_MAP, BOTTOM_FACE_MASK, NORTH_FACE_MASK, SOUTH_FACE_MASK,
        TOP_FACE_MASK, VALID_BOTTOM_FACES, VALID_SOUTH_FACES, VALID_TOP_FACES,
        Y_EXTERIOR_FACE_LOOKUP,
    },
};

use bevy::prelude::Mesh;
use bevy::{
    math::UVec3,
    prelude::Vec3,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub type BlockIndex = u8;

pub struct Mesher {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl Mesher {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn mesh_grid(mut self, grid: &Grid) -> Self {
        for z in 0..grid.size.z {
            for y in 0..grid.size.y {
                for x in 0..grid.size.x {
                    let pos = UVec3 { x, y, z };
                    let idx = grid.index_at_pos(pos);
                    let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                    let shape_descriptor = shape.to_shape_descriptor();
                    let idx = SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP[shape_descriptor as usize];
                    let tris =
                        &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP[shape_descriptor as usize];
                    self.add_vertices_at_pos(tris, pos);

                    // Bottom
                    {
                        let flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize] & 0b1111;
                        let bottom_shape_descriptor = if y == 0 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::SixSixth).to_shape_descriptor() // pretend it's a full block so bottom of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos - UVec3 { x: 0, y: 1, z: 0 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let bottom_flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[bottom_shape_descriptor as usize] & 0b1111_0000) >> 4;

                        let result_flag = if flag == 0 { bottom_flag } else { !flag & bottom_flag };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(&match result {
                            0b1111 => 
                                vec![
                                    [
                                        UVec3 { x: 0, y: 0, z: 0 },
                                        UVec3 { x: 0, y: 0, z: 1 },
                                        UVec3 { x: 1, y: 0, z: 1 },
                                    ],
                                    [
                                        UVec3 { x: 0, y: 0, z: 0 },
                                        UVec3 { x: 1, y: 0, z: 1 },
                                        UVec3 { x: 1, y: 0, z: 0 },
                                    ],
                                ]
                            ,
                            0b0100 => 
                                vec![[
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                ]],
                            0b1000 => 
                                vec![[
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                ]],
                            0b0001 => 
                                vec![[
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                ]],
                            0b0010 => 
                                vec![[
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                ]],
                            _ => vec![]
                        }, pos);
                    }

                    if x == 0 && y == 0 && z == 0 {
                        let shape = &BLOCK_INDEX_TO_SHAPE_MAP[0b1111_1000_usize];
                        println!("{:?}", shape);
                        let shape_descriptor = shape.to_shape_descriptor();
                        let flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize];
                        println!("upside down flag {:#18b}", (flag & 0b1111_0000) >> 4)
                    }

                    // Top
                    {
                        let flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize] & 0b1111;
                        let top_shape_descriptor = if y == grid.size.y - 1 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::ZeroSixth).to_shape_descriptor() // pretend it's a full block so top of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos + UVec3 { x: 0, y: 1, z: 0 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let top_flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[top_shape_descriptor as usize] & 0b1111;

                        let result_flag = if flag == 0 { top_flag } else { !flag & top_flag };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(&match result {
                            0b1111 => 
                                vec![
                                    [
                                        UVec3 { x: 0, y: 1, z: 0 },
                                        UVec3 { x: 1, y: 1, z: 1 },
                                        UVec3 { x: 0, y: 1, z: 1 },
                                    ],
                                    [
                                        UVec3 { x: 0, y: 1, z: 0 },
                                        UVec3 { x: 1, y: 1, z: 0 },
                                        UVec3 { x: 1, y: 1, z: 1 },
                                    ],
                                ]
                            ,
                            0b0100 => 
                                vec![[
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                ]],
                            0b1000 => 
                                vec![[
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                ]],
                            0b0001 => 
                                vec![[
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                ]],
                            0b0010 => 
                                vec![[
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                ]],
                            _ => vec![]
                        }, pos);
                    }

                    // Top
                    // {
                    //     let idx_mask = idx & TOP_FACE_MASK;
                    //     let top_idx = if y == grid.size.y - 1 {
                    //         0
                    //     } else {
                    //         let idx = grid.index_at_pos(pos + UVec3 { x: 0, y: 1, z: 0 });
                    //         let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                    //         SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP
                    //             [shape.to_shape_descriptor() as usize]
                    //     };
                    //     let top_idx_mask = (top_idx & BOTTOM_FACE_MASK) << 4;
                    //     if top_idx_mask != TOP_FACE_MASK
                    //         && VALID_TOP_FACES.contains(&top_idx_mask)
                    //         && VALID_TOP_FACES.contains(&idx_mask)
                    //     {
                    //         let face_idx = idx_mask & !top_idx_mask;
                    //         self.add_vertices_at_pos(
                    //             &Y_EXTERIOR_FACE_LOOKUP[face_idx as usize],
                    //             pos,
                    //         );
                    //     }
                    // }
                    // Bottom
                    // {
                    //     let idx_mask = idx & BOTTOM_FACE_MASK;
                        // let bottom_idx = if y == 0 {
                        //     0b1111_1111 // pretend it's a full block so bottom of the chunks doesn't get rendered
                        // } else {
                        //     let idx = grid.index_at_pos(pos - UVec3 { x: 0, y: 1, z: 0 });
                        //     let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                        //     SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP
                        //         [shape.to_shape_descriptor() as usize]
                        // };
                    //     let bottom_idx_mask = (bottom_idx & TOP_FACE_MASK) >> 4;
                    //     if bottom_idx_mask != BOTTOM_FACE_MASK
                    //         && VALID_BOTTOM_FACES.contains(&bottom_idx_mask)
                    //         && VALID_BOTTOM_FACES.contains(&idx_mask)
                    //     {
                    //         let face_idx = idx_mask & !bottom_idx_mask;
                    //         if x == 31 && y == 75 && z == 47 {
                    //             println!("shape: {:?}", shape);
                    //             println!("idx_mask: {:#10b}", idx_mask);
                    //             println!("bottom_idx_mask: {:#10b}", bottom_idx_mask);
                    //             println!("face_idx: {:#10b}", face_idx);
                    //             println!("face_idx: {}", face_idx);
                    //         }
                    //         self.add_vertices_at_pos(
                    //             &Y_EXTERIOR_FACE_LOOKUP[face_idx as usize],
                    //             pos,
                    //         );
                    //     }
                    // }

                    // South
                    // {
                    //     let idx_mask = idx & NORTH_FACE_MASK;
                    //     let north_idx = if z == 0 {
                    //         0
                    //     } else {
                    //         let idx = grid.index_at_pos(pos - UVec3 { x: 0, y: 0, z: 1 });
                    //         let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                    //         SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP
                    //             [shape.to_shape_descriptor() as usize]
                    //     };
                    //     let north_idx_mask = (north_idx & NORTH_FACE_MASK) << 4;
                    //     if north_idx_mask != TOP_FACE_MASK
                    //         && VALID_NORTH_FACES.contains(&north_idx_mask)
                    //         && VALID_NORTH_FACES.contains(&idx_mask)
                    //     {
                    //         let face_idx = idx_mask & !north_idx_mask;
                    //         self.add_vertices_at_pos(
                    //             &Y_EXTERIOR_FACE_LOOKUP[face_idx as usize],
                    //             pos,
                    //         );
                    //     }
                    // }
                }
            }
        }
        self
    }

    pub fn mesh(&mut self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh
    }

    pub fn add_vertices_at_pos(&mut self, triangles: &Vec<[UVec3; 3]>, pos: UVec3) {
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

            let normal = Self::normal(tri_vertices[0], tri_vertices[2], tri_vertices[1]).to_array();

            // All three vertices should share the same normal because that's how lowpoly works
            self.normals.append(&mut vec![normal, normal, normal]);
            self.uvs
                .append(&mut vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]);
            let next_index = match self.indices.last() {
                Some(n) => n + 1,
                None => 0,
            };
            self.indices
                .append(&mut [next_index, next_index + 1, next_index + 2].into());
        }
    }

    //  Cube layout:
    //       6----7
    //      /|   /|
    //     5-+--4 |
    //     | |  | |       y
    //     | 2--+-3       | z
    //     |/   |/     x__|/
    //     1----0
    //

    // West: 0b1010_1010
    // East: 0b0101_0101

    // North: 0b1100_1100
    // South: 0b0011_0011
    //  Test Cube layout:
    //       7----6
    //      /|   /|
    //     5-+--4 |
    //     | |  | |       y
    //     | 3--+-2       | z
    //     |/   |/     x__|/
    //     1----0
    //

    pub fn normal(a: Vec3, b: Vec3, c: Vec3) -> Vec3 {
        (c - a).cross(b - a).normalize()
    }

    fn get_middle_flag_corner(flag: u32) -> u32 {
        match flag {
            0b1011 => 0b0001,
            0b0111 => 0b0010,
            0b1110 => 0b0100,
            0b1101 => 0b1000,
            e => e
        }
    }
}

pub struct Grid {
    pub size: UVec3,
    pub blocks: Vec<BlockIndex>,
}

impl Grid {
    pub fn new(size: UVec3) -> Self {
        let capacity = size.x * size.y * size.z;
        Grid {
            size,
            blocks: vec![0; capacity as usize],
        }
    }

    pub fn generate(mut self, generator: &impl ChunkGenerator) -> Self {
        generator.generate(&mut self);
        self
    }

    pub fn index_at_pos(&self, pos: UVec3) -> BlockIndex {
        self.blocks[self.pos_idx(pos)]
    }

    pub const fn pos_idx(&self, pos: UVec3) -> usize {
        ((self.size.z * self.size.x * pos.y) + (self.size.z * pos.x) + pos.z) as usize
    }
}

// Draft 1:
// For each adjacent face, compute an excluding index so that we can detect which triangles to mesh on the exterior of the cubes.
// Example: We have a 3/6 rolled on top of a 6/6. Only half the top of the cube is visible.
// - Compute the top index so that we have a value which represent only the top face: index &= 0b1111_0000
// - Transform the 3/6 index so that it represents the vertices as being on top of the bottom cube: (index & BOTTOM_FACE) << 4 // this one is easy but other faces will require more bitwise trickery
// - Exclude the two indices: bottom_indice & top_indice
// - The resulting value will only keep either index 4, 5, 6 or 7 as the diagonal is also excluded. But the remaining index represent the triangle to mesh.
//     Whatever the top block is, there's only a few possible outcomes:
//     - Either there is only a specific bit so we need to render a triangle on that edge
//     - Either the value is the same as before exclusion, meaning the full face needs to be meshed (does need to be a special case tho)
//     - Either the value is 0, meaning the block above completely hides the top face(s) of the bottom block so there's nothing to mesh
//     - Either the value is anything else (will contains either 2 or 3 bits), meaning there's only 1 or 2 common vertices, which will never hide anything, so that must be considered as a full face
//     Important: all of the above outcomes do not need any special case. The table that we'll compute will already cover all of this
