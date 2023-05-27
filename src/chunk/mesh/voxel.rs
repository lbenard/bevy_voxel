use bevy::prelude::{IVec3, UVec3};

use crate::{
    chunk::Grid,
    terrain::block::{
        material::Material,
        shape::{
            Shape, ShapeDescriptor, SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP,
            SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP,
        },
        Block,
    },
};

use super::ChunkMesh;

pub struct Voxel {
    pub block: Option<Block>,
    pub position: UVec3,
}

impl Grid {
    pub fn voxel_at_pos(&self, pos: UVec3) -> Voxel {
        Voxel::new(self.block_at_pos(pos.as_ivec3()), pos)
    }
}

impl Voxel {
    pub fn new(block: Option<Block>, position: UVec3) -> Self {
        Self { block, position }
    }

    pub fn mesh(&self, chunk_mesh: &mut ChunkMesh, grid: &Grid) {
        let shape = self.block.map(|block| block.shape).unwrap_or(Shape::EMPTY);

        let shape_descriptor: ShapeDescriptor = shape.into();
        if let Some(block) = self.block {
            let tris = &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP[shape_descriptor.0 as usize];
            chunk_mesh.add_vertices_at_pos(self.position, tris, &block.material);

            for side in SIDES.iter() {
                let side_descriptor =
                    SideDescriptor::from_shape_descriptor(&shape_descriptor, *side);
                if side_descriptor.descriptor == 0 {
                    continue;
                }
                let adjacent_block = grid.block_at_pos(side.adjacent_position(self.position));
                let adjacent_shape = adjacent_block
                    .map(|block| block.shape)
                    .unwrap_or(Shape::EMPTY);
                let adjacent_side_descriptor =
                    SideDescriptor::from_shape_descriptor(&adjacent_shape.into(), side.opposite());

                let result_side_descriptor = SideDescriptor::from_adjacent_sides(
                    &side_descriptor,
                    &adjacent_side_descriptor,
                );

                result_side_descriptor.mesh_side(chunk_mesh, self.position, &block.material);
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Side {
    North,
    South,
    Top,
    Bottom,
    West,
    East,
}

impl Side {
    fn opposite(&self) -> Side {
        match self {
            Side::North => Side::South,
            Side::South => Side::North,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::West => Side::East,
            Side::East => Side::West,
        }
    }

    fn adjacent_position(&self, pos: UVec3) -> IVec3 {
        let pos = pos.as_ivec3();
        match self {
            Side::North => IVec3 {
                x: pos.x,
                y: pos.y,
                z: pos.z + 1,
            },
            Side::South => IVec3 {
                x: pos.x,
                y: pos.y,
                z: pos.z - 1,
            },
            Side::Top => IVec3 {
                x: pos.x,
                y: pos.y + 1,
                z: pos.z,
            },
            Side::Bottom => IVec3 {
                x: pos.x,
                y: pos.y - 1,
                z: pos.z,
            },
            Side::West => IVec3 {
                x: pos.x + 1,
                y: pos.y,
                z: pos.z,
            },
            Side::East => IVec3 {
                x: pos.x - 1,
                y: pos.y,
                z: pos.z,
            },
        }
    }
}

pub const SIDES: [Side; 6] = [
    Side::North,
    Side::South,
    Side::Top,
    Side::Bottom,
    Side::West,
    Side::East,
];

#[derive(Debug)]
struct SideDescriptor {
    side: Side,
    descriptor: u8,
}

impl SideDescriptor {
    pub fn from_shape_descriptor(shape_descriptor: &ShapeDescriptor, side: Side) -> Self {
        let shift: usize = match side {
            Side::Bottom => 0,
            Side::Top => 4,
            Side::West => 8,
            Side::South => 12,
            Side::East => 16,
            Side::North => 20,
        };
        let descriptor = (SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape_descriptor.0 as usize]
            & (0b1111 << shift) as u32)
            >> shift;
        Self {
            side,
            descriptor: descriptor as u8,
        }
    }

    pub fn from_adjacent_sides(side: &SideDescriptor, adjacent_side: &SideDescriptor) -> Self {
        let result = if adjacent_side.descriptor == 0 {
            side.descriptor
        } else {
            !adjacent_side.descriptor & side.descriptor
        };

        let result = match result {
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
        };

        Self {
            side: side.side,
            descriptor: result,
        }
    }

    pub fn mesh_side(&self, chunk_mesh: &mut ChunkMesh, pos: UVec3, material: &Material) {
        let vertices: Vec<[UVec3; 3]> = match self.side {
            Side::Bottom => match self.descriptor {
                0b1111 => vec![
                    [
                        UVec3 { x: 0, y: 0, z: 0 },
                        UVec3 { x: 1, y: 0, z: 1 },
                        UVec3 { x: 0, y: 0, z: 1 },
                    ],
                    [
                        UVec3 { x: 0, y: 0, z: 0 },
                        UVec3 { x: 1, y: 0, z: 0 },
                        UVec3 { x: 1, y: 0, z: 1 },
                    ],
                ],
                0b0001 => vec![[
                    UVec3 { x: 0, y: 0, z: 0 },
                    UVec3 { x: 1, y: 0, z: 0 },
                    UVec3 { x: 0, y: 0, z: 1 },
                ]],
                0b0010 => vec![[
                    UVec3 { x: 0, y: 0, z: 1 },
                    UVec3 { x: 0, y: 0, z: 0 },
                    UVec3 { x: 1, y: 0, z: 1 },
                ]],
                0b0100 => vec![[
                    UVec3 { x: 1, y: 0, z: 1 },
                    UVec3 { x: 0, y: 0, z: 1 },
                    UVec3 { x: 1, y: 0, z: 0 },
                ]],
                0b1000 => vec![[
                    UVec3 { x: 1, y: 0, z: 0 },
                    UVec3 { x: 1, y: 0, z: 1 },
                    UVec3 { x: 0, y: 0, z: 0 },
                ]],
                _ => vec![],
            },
            Side::Top => match self.descriptor {
                0b1111 => vec![
                    [
                        UVec3 { x: 0, y: 1, z: 0 },
                        UVec3 { x: 0, y: 1, z: 1 },
                        UVec3 { x: 1, y: 1, z: 1 },
                    ],
                    [
                        UVec3 { x: 0, y: 1, z: 0 },
                        UVec3 { x: 1, y: 1, z: 1 },
                        UVec3 { x: 1, y: 1, z: 0 },
                    ],
                ],
                0b0001 => vec![[
                    UVec3 { x: 0, y: 1, z: 0 },
                    UVec3 { x: 0, y: 1, z: 1 },
                    UVec3 { x: 1, y: 1, z: 0 },
                ]],
                0b0010 => vec![[
                    UVec3 { x: 0, y: 1, z: 1 },
                    UVec3 { x: 1, y: 1, z: 1 },
                    UVec3 { x: 0, y: 1, z: 0 },
                ]],
                0b0100 => vec![[
                    UVec3 { x: 1, y: 1, z: 1 },
                    UVec3 { x: 1, y: 1, z: 0 },
                    UVec3 { x: 0, y: 1, z: 1 },
                ]],
                0b1000 => vec![[
                    UVec3 { x: 1, y: 1, z: 0 },
                    UVec3 { x: 0, y: 1, z: 0 },
                    UVec3 { x: 1, y: 1, z: 1 },
                ]],
                _ => vec![],
            },
            Side::West => match self.descriptor {
                0b1111 => vec![
                    [
                        UVec3 { x: 1, y: 0, z: 0 },
                        UVec3 { x: 1, y: 1, z: 1 },
                        UVec3 { x: 1, y: 0, z: 1 },
                    ],
                    [
                        UVec3 { x: 1, y: 0, z: 0 },
                        UVec3 { x: 1, y: 1, z: 0 },
                        UVec3 { x: 1, y: 1, z: 1 },
                    ],
                ],
                0b0001 => vec![[
                    UVec3 { x: 1, y: 0, z: 0 },
                    UVec3 { x: 1, y: 1, z: 0 },
                    UVec3 { x: 1, y: 0, z: 1 },
                ]],
                0b0010 => vec![[
                    UVec3 { x: 1, y: 1, z: 0 },
                    UVec3 { x: 1, y: 1, z: 1 },
                    UVec3 { x: 1, y: 0, z: 0 },
                ]],
                0b0100 => vec![[
                    UVec3 { x: 1, y: 1, z: 1 },
                    UVec3 { x: 1, y: 0, z: 1 },
                    UVec3 { x: 1, y: 1, z: 0 },
                ]],
                0b1000 => vec![[
                    UVec3 { x: 1, y: 0, z: 1 },
                    UVec3 { x: 1, y: 0, z: 0 },
                    UVec3 { x: 1, y: 1, z: 1 },
                ]],
                _ => vec![],
            },
            Side::East => match self.descriptor {
                0b1111 => vec![
                    [
                        UVec3 { x: 0, y: 0, z: 0 },
                        UVec3 { x: 0, y: 0, z: 1 },
                        UVec3 { x: 0, y: 1, z: 1 },
                    ],
                    [
                        UVec3 { x: 0, y: 0, z: 0 },
                        UVec3 { x: 0, y: 1, z: 1 },
                        UVec3 { x: 0, y: 1, z: 0 },
                    ],
                ],
                0b0001 => vec![[
                    UVec3 { x: 0, y: 0, z: 0 },
                    UVec3 { x: 0, y: 0, z: 1 },
                    UVec3 { x: 0, y: 1, z: 0 },
                ]],
                0b0010 => vec![[
                    UVec3 { x: 0, y: 1, z: 0 },
                    UVec3 { x: 0, y: 0, z: 0 },
                    UVec3 { x: 0, y: 1, z: 1 },
                ]],
                0b0100 => vec![[
                    UVec3 { x: 0, y: 1, z: 1 },
                    UVec3 { x: 0, y: 1, z: 0 },
                    UVec3 { x: 0, y: 0, z: 1 },
                ]],
                0b1000 => vec![[
                    UVec3 { x: 0, y: 0, z: 1 },
                    UVec3 { x: 0, y: 1, z: 1 },
                    UVec3 { x: 0, y: 0, z: 0 },
                ]],
                _ => vec![],
            },
            Side::North => match self.descriptor {
                0b1111 => vec![
                    [
                        UVec3 { x: 0, y: 0, z: 1 },
                        UVec3 { x: 1, y: 0, z: 1 },
                        UVec3 { x: 1, y: 1, z: 1 },
                    ],
                    [
                        UVec3 { x: 0, y: 0, z: 1 },
                        UVec3 { x: 1, y: 1, z: 1 },
                        UVec3 { x: 0, y: 1, z: 1 },
                    ],
                ],
                0b0001 => vec![[
                    UVec3 { x: 0, y: 0, z: 1 },
                    UVec3 { x: 1, y: 0, z: 1 },
                    UVec3 { x: 0, y: 1, z: 1 },
                ]],
                0b0010 => vec![[
                    UVec3 { x: 1, y: 0, z: 1 },
                    UVec3 { x: 1, y: 1, z: 1 },
                    UVec3 { x: 0, y: 0, z: 1 },
                ]],
                0b0100 => vec![[
                    UVec3 { x: 1, y: 1, z: 1 },
                    UVec3 { x: 0, y: 1, z: 1 },
                    UVec3 { x: 1, y: 0, z: 1 },
                ]],
                0b1000 => vec![[
                    UVec3 { x: 0, y: 1, z: 1 },
                    UVec3 { x: 0, y: 0, z: 1 },
                    UVec3 { x: 1, y: 1, z: 1 },
                ]],
                _ => vec![],
            },
            Side::South => match self.descriptor {
                0b1111 => vec![
                    [
                        UVec3 { x: 0, y: 0, z: 0 },
                        UVec3 { x: 1, y: 1, z: 0 },
                        UVec3 { x: 1, y: 0, z: 0 },
                    ],
                    [
                        UVec3 { x: 0, y: 0, z: 0 },
                        UVec3 { x: 0, y: 1, z: 0 },
                        UVec3 { x: 1, y: 1, z: 0 },
                    ],
                ],
                0b0001 => vec![[
                    UVec3 { x: 0, y: 0, z: 0 },
                    UVec3 { x: 0, y: 1, z: 0 },
                    UVec3 { x: 1, y: 0, z: 0 },
                ]],
                0b0010 => vec![[
                    UVec3 { x: 1, y: 0, z: 0 },
                    UVec3 { x: 0, y: 0, z: 0 },
                    UVec3 { x: 1, y: 1, z: 0 },
                ]],
                0b0100 => vec![[
                    UVec3 { x: 1, y: 1, z: 0 },
                    UVec3 { x: 1, y: 0, z: 0 },
                    UVec3 { x: 0, y: 1, z: 0 },
                ]],
                0b1000 => vec![[
                    UVec3 { x: 0, y: 1, z: 0 },
                    UVec3 { x: 1, y: 1, z: 0 },
                    UVec3 { x: 0, y: 0, z: 0 },
                ]],
                _ => vec![],
            },
        };

        chunk_mesh.add_vertices_at_pos(pos, &vertices, material);
    }
}
