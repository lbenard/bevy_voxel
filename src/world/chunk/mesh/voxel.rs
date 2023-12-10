use bevy::prelude::UVec3;

use crate::world::{
    chunk::Terrain,
    voxel::{
        material::Material,
        shape::{
            Shape, ShapeDescriptor, SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP,
            SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP,
        },
        Side, VoxelDescriptor,
    },
    World, WorldChunk,
};

use super::ChunkMesh;

pub struct VoxelMesh {
    pub voxel: VoxelDescriptor,
    pub position: UVec3,
}

impl Terrain {
    pub fn voxel_mesh_at_pos(&self, pos: UVec3) -> Option<VoxelMesh> {
        let voxel_descriptor = self.voxel_at_pos(pos.as_ivec3());
        voxel_descriptor.map(|vd| VoxelMesh::new(vd, pos))
    }
}

impl VoxelMesh {
    pub fn new(voxel: VoxelDescriptor, position: UVec3) -> Self {
        Self { voxel, position }
    }

    pub fn mesh(&self, chunk_mesh: &mut ChunkMesh, chunk: WorldChunk, world: &World) {
        let shape_descriptor: ShapeDescriptor = self.voxel.shape.into();
        let tris = &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP[shape_descriptor.0 as usize];
        chunk_mesh.add_vertices_at_pos(self.position, tris, &self.voxel.material);

        for side in SIDES.iter() {
            let side_descriptor = SideDescriptor::from_shape_descriptor(&shape_descriptor, *side);
            if side_descriptor.descriptor == 0 {
                continue;
            }
            let adjacent_voxel = world
                .get_voxel(chunk.read().absolute_position + side.adjacent_position(self.position));

            let adjacent_shape = adjacent_voxel
                .map(|voxel| voxel.shape)
                .unwrap_or(Shape::EMPTY);
            let adjacent_side_descriptor =
                SideDescriptor::from_shape_descriptor(&adjacent_shape.into(), side.opposite());

            let result_side_descriptor =
                SideDescriptor::from_adjacent_sides(&side_descriptor, &adjacent_side_descriptor);

            result_side_descriptor.mesh_side(chunk_mesh, self.position, &self.voxel.material);
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
