use crate::chunk::Grid;

use super::block::shape::{
    Rotation, Shape, Volume, BLOCK_INDEX_TO_SHAPE_MAP, SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP,
    SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP,
};

use bevy::prelude::{Color, Mesh};
use bevy::{
    math::UVec3,
    prelude::Vec3,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use rand::Rng;

pub struct Mesher {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    colors: Vec<[f32; 4]>,
    // uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl Mesher {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            colors: Vec::new(),
            // uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn mesh_grid(mut self, grid: &Grid) -> Self {
        for z in 0..grid.size.z {
            for x in 0..grid.size.x {
                for y in 0..grid.size.y {
                    let pos = UVec3 { x, y, z };
                    let block = grid.block_at_pos(pos);
                    let shape = block
                        .and_then(|block| Some(block.shape))
                        .unwrap_or(Shape::EMPTY);
                    if shape.volume == Volume::SixSixth {
                        continue;
                    }

                    let shape_descriptor = shape.to_shape_descriptor();
                    let tris =
                        &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP[shape_descriptor as usize];
                    self.add_vertices_at_pos(pos, tris);

                    // Bottom
                    {
                        let flag = SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor as usize]
                            & 0b1111 as u32;
                        let bottom_shape_descriptor = if y == 0 {
                            Shape::new(Rotation::FacingNorth0Degrees, Volume::SixSixth)
                                .to_shape_descriptor() // pretend it's a full block so bottom of the chunks doesn't get rendered
                        } else {
                            let block = grid.block_at_pos(pos - UVec3 { x: 0, y: 1, z: 0 });
                            block
                                .and_then(|block| Some(block.shape))
                                .unwrap_or(Shape::EMPTY)
                                .to_shape_descriptor()
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
                            pos,
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
                            let block = grid.block_at_pos(pos + UVec3 { x: 0, y: 1, z: 0 });
                            block
                                .and_then(|block| Some(block.shape))
                                .unwrap_or(Shape::EMPTY)
                                .to_shape_descriptor()
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
                            pos,
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
                            let block = grid.block_at_pos(pos + UVec3 { x: 1, y: 0, z: 0 });
                            block
                                .and_then(|block| Some(block.shape))
                                .unwrap_or(Shape::EMPTY)
                                .to_shape_descriptor()
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
                            pos,
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
                            let block = grid.block_at_pos(pos - UVec3 { x: 1, y: 0, z: 0 });
                            block
                                .and_then(|block| Some(block.shape))
                                .unwrap_or(Shape::EMPTY)
                                .to_shape_descriptor()
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
                            pos,
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
                            let block = grid.block_at_pos(pos + UVec3 { x: 0, y: 0, z: 1 });
                            block
                                .and_then(|block| Some(block.shape))
                                .unwrap_or(Shape::EMPTY)
                                .to_shape_descriptor()
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
                            pos,
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
                            let block = grid.block_at_pos(pos - UVec3 { x: 0, y: 0, z: 1 });
                            block
                                .and_then(|block| Some(block.shape))
                                .unwrap_or(Shape::EMPTY)
                                .to_shape_descriptor()
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
                            pos,
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
        // mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.colors.clone());
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs.clone());
        mesh.set_indices(Some(Indices::U32(self.indices.clone())));
        mesh
    }

    pub fn add_vertices_at_pos(&mut self, pos: UVec3, triangles: &Vec<[UVec3; 3]>) {
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
            let color = Color::rgba(1.0, 0.0, 0.0, 1.0).as_rgba_f32();
            self.colors.append(&mut vec![color, color, color]);
            // self.uvs
            //     .append(&mut vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]]);
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
            // TODO: Z ????
        }
    }
}
