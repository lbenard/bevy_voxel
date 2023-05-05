use crate::terrain::block::Shape;

use super::{
    block::{
        Rotation, Volume, BLOCK_INDEX_TO_SHAPE_MAP, SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP,
        SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP,
    },
    generator::ChunkGenerator,
};

use bevy::prelude::{IVec3, Mesh};
use bevy::{
    math::UVec3,
    prelude::Vec3,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub type BlockIndex = u8;

pub struct Chunk {
    pub absolute_position: IVec3,
    pub mesh: Mesh,
}

impl Chunk {
    pub fn new(absolute_position: IVec3, mesh: Mesh) -> Self {
        Self {
            absolute_position,
            mesh,
        }
    }
}

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
            for x in 0..grid.size.x {
                // let start_y = (0..grid.size.y)
                //     .find(|y| {
                //         BLOCK_INDEX_TO_SHAPE_MAP[grid.index_at_pos(UVec3 { x, y: *y, z }) as usize]
                //             .volume
                //             != Volume::ZeroSixth
                //     })
                //     .unwrap();
                // let end_y = ((0..grid.size.y)
                //     .rfind(|y| {
                //         BLOCK_INDEX_TO_SHAPE_MAP[grid.index_at_pos(UVec3 { x, y: *y, z }) as usize]
                //             .volume
                //             != Volume::ZeroSixth
                //     })
                //     .unwrap()
                //     + 2)
                // .min(grid.size.y - 1);
                // println!("{start_y} {end_y}");
                for y in 0..grid.size.y {
                    let pos = UVec3 { x, y, z };
                    let idx = grid.index_at_pos(pos);
                    let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                    if shape.volume == Volume::SixSixth {
                        continue;
                    }

                    let shape_descriptor = shape.to_shape_descriptor();
                    let tris =
                        &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP[shape_descriptor as usize];
                    self.add_vertices_at_pos(tris, pos);

                    // Bottom
                    {
                        let flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & 0b1111 as u32;
                        let bottom_shape_descriptor = if y == 0 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::SixSixth)
                                .to_shape_descriptor() // pretend it's a full block so bottom of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos - UVec3 { x: 0, y: 1, z: 0 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let bottom_flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP
                            [bottom_shape_descriptor as usize]
                            & (0b1111 << 4) as u32)
                            >> 4;

                        let result_flag = if flag == 0 {
                            bottom_flag
                        } else {
                            !flag & bottom_flag
                        };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(
                            &match result {
                                0b1111 => vec![
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
                                ],
                                0b0001 => vec![[
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                ]],
                                0b0010 => vec![[
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                ]],
                                0b0100 => vec![[
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                ]],
                                0b1000 => vec![[
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                ]],
                                _ => vec![],
                            },
                            pos,
                        );
                    }

                    // Top
                    {
                        let flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & (0b1111 << 4) as u32)
                            >> 4;
                        let top_shape_descriptor = if y == grid.size.y - 1 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::ZeroSixth)
                                .to_shape_descriptor() // pretend it's a full block so top of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos + UVec3 { x: 0, y: 1, z: 0 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let top_flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP
                            [top_shape_descriptor as usize]
                            & 0b1111 as u32;

                        let result_flag = if flag == 0 {
                            top_flag
                        } else {
                            !flag & top_flag
                        };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(
                            &match result {
                                0b1111 => vec![
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
                                ],
                                0b0001 => vec![[
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                ]],
                                0b0010 => vec![[
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                ]],
                                0b0100 => vec![[
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                ]],
                                0b1000 => vec![[
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                ]],
                                _ => vec![],
                            },
                            pos,
                        );
                    }

                    // West
                    {
                        let flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & (0b1111 << 8) as u32)
                            >> 8;
                        let west_shape_descriptor = if x == grid.size.x - 1 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::ZeroSixth)
                                .to_shape_descriptor() // pretend it's a full block so west of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos + UVec3 { x: 1, y: 0, z: 0 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let west_flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP
                            [west_shape_descriptor as usize]
                            & (0b1111 << 16) as u32)
                            >> 16;

                        let result_flag = if flag == 0 {
                            west_flag
                        } else {
                            !flag & west_flag
                        };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(
                            &match result {
                                0b1111 => vec![
                                    [
                                        UVec3 { x: 1, y: 0, z: 0 },
                                        UVec3 { x: 1, y: 0, z: 1 },
                                        UVec3 { x: 1, y: 1, z: 1 },
                                    ],
                                    [
                                        UVec3 { x: 1, y: 0, z: 0 },
                                        UVec3 { x: 1, y: 1, z: 1 },
                                        UVec3 { x: 1, y: 1, z: 0 },
                                    ],
                                ],
                                0b0001 => vec![[
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                ]],
                                0b0010 => vec![[
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                ]],
                                0b0100 => vec![[
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                ]],
                                0b1000 => vec![[
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                ]],
                                _ => vec![],
                            },
                            pos,
                        );
                    }

                    // East
                    {
                        let flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & (0b1111 << 16) as u32)
                            >> 16;
                        let east_shape_descriptor = if x == 0 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::ZeroSixth)
                                .to_shape_descriptor() // pretend it's a full block so east of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos - UVec3 { x: 1, y: 0, z: 0 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let east_flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP
                            [east_shape_descriptor as usize]
                            & (0b1111 << 8) as u32)
                            >> 8;

                        let result_flag = if flag == 0 {
                            east_flag
                        } else {
                            !flag & east_flag
                        };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(
                            &match result {
                                0b1111 => vec![
                                    [
                                        UVec3 { x: 0, y: 0, z: 0 },
                                        UVec3 { x: 0, y: 1, z: 1 },
                                        UVec3 { x: 0, y: 0, z: 1 },
                                    ],
                                    [
                                        UVec3 { x: 0, y: 0, z: 0 },
                                        UVec3 { x: 0, y: 1, z: 0 },
                                        UVec3 { x: 0, y: 1, z: 1 },
                                    ],
                                ],
                                0b0001 => vec![[
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                ]],
                                0b0010 => vec![[
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                ]],
                                0b0100 => vec![[
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                ]],
                                0b1000 => vec![[
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                ]],
                                _ => vec![],
                            },
                            pos,
                        );
                    }

                    // North
                    {
                        let flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & (0b1111 << 20) as u32)
                            >> 20;
                        let north_shape_descriptor = if z == grid.size.z - 1 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::ZeroSixth)
                                .to_shape_descriptor() // pretend it's a full block so north of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos + UVec3 { x: 0, y: 0, z: 1 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let north_flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP
                            [north_shape_descriptor as usize]
                            & (0b1111 << 12) as u32)
                            >> 12;

                        let result_flag = if flag == 0 {
                            north_flag
                        } else {
                            !flag & north_flag
                        };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(
                            &match result {
                                0b1111 => vec![
                                    [
                                        UVec3 { x: 0, y: 0, z: 1 },
                                        UVec3 { x: 1, y: 1, z: 1 },
                                        UVec3 { x: 1, y: 0, z: 1 },
                                    ],
                                    [
                                        UVec3 { x: 0, y: 0, z: 1 },
                                        UVec3 { x: 0, y: 1, z: 1 },
                                        UVec3 { x: 1, y: 1, z: 1 },
                                    ],
                                ],
                                0b0001 => vec![[
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                ]],
                                0b0010 => vec![[
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                ]],
                                0b0100 => vec![[
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 0, z: 1 },
                                    UVec3 { x: 0, y: 1, z: 1 },
                                ]],
                                0b1000 => vec![[
                                    UVec3 { x: 0, y: 1, z: 1 },
                                    UVec3 { x: 1, y: 1, z: 1 },
                                    UVec3 { x: 0, y: 0, z: 1 },
                                ]],
                                _ => vec![],
                            },
                            pos,
                        );
                    }

                    // South
                    {
                        let flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & (0b1111 << 12) as u32)
                            >> 12;
                        let south_shape_descriptor = if z == 0 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::ZeroSixth)
                                .to_shape_descriptor() // pretend it's a full block so south of the chunks doesn't get rendered
                        } else {
                            let idx = grid.index_at_pos(pos - UVec3 { x: 0, y: 0, z: 1 });
                            let shape = &BLOCK_INDEX_TO_SHAPE_MAP[idx as usize];
                            shape.to_shape_descriptor()
                        };
                        let south_flag = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP
                            [south_shape_descriptor as usize]
                            & (0b1111 << 20) as u32)
                            >> 20;

                        let result_flag = if flag == 0 {
                            south_flag
                        } else {
                            !flag & south_flag
                        };

                        let result = Self::get_middle_flag_corner(result_flag);
                        self.add_vertices_at_pos(
                            &match result {
                                0b1111 => vec![
                                    [
                                        UVec3 { x: 0, y: 0, z: 0 },
                                        UVec3 { x: 1, y: 0, z: 0 },
                                        UVec3 { x: 1, y: 1, z: 0 },
                                    ],
                                    [
                                        UVec3 { x: 0, y: 0, z: 0 },
                                        UVec3 { x: 1, y: 1, z: 0 },
                                        UVec3 { x: 0, y: 1, z: 0 },
                                    ],
                                ],
                                0b0001 => vec![[
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                ]],
                                0b0010 => vec![[
                                    UVec3 { x: 1, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                ]],
                                0b0100 => vec![[
                                    UVec3 { x: 1, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 1, y: 0, z: 0 },
                                ]],
                                0b1000 => vec![[
                                    UVec3 { x: 0, y: 1, z: 0 },
                                    UVec3 { x: 0, y: 0, z: 0 },
                                    UVec3 { x: 1, y: 1, z: 0 },
                                ]],
                                _ => vec![],
                            },
                            pos,
                        );
                    }
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
    // Top: 0b1111_0000
    // Bottom: 0b0000_1111
    // West: 0b1010_1010
    // East: 0b0101_0101

    // Face flag rotation order per direction:
    // x: 0 -> 4 -> 6 -> 2 or 1 -> 5 -> 7 -> 3
    // y: 0 -> 2 -> 3 -> 1 or 4 -> 6 -> 7 -> 5
    // z: 0 -> 1 -> 5 -> 4 or 2 -> 3 -> 7 -> 6
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

    // TODO: can probably be replaced by some bitwise trickery
    fn get_middle_flag_corner(flag: u32) -> u32 {
        match flag {
            // x axis
            0b0001_0101 => 0b0001,
            0b0101_0001 => 0b0010,
            0b0101_0100 => 0b0100,
            0b0100_0101 => 0b1000,
            // y axis
            0b1011 => 0b0001,
            0b0111 => 0b0010,
            0b1110 => 0b0100,
            0b1101 => 0b1000,
            e => e,
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

    pub fn generate(mut self, origin: IVec3, generator: &impl ChunkGenerator) -> Self {
        generator.generate(origin, &mut self);
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
