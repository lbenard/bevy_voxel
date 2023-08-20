use crate::world::{
    chunk::{Grid, CHUNK_SIZE},
    voxel::{
        material,
        shape::{ShapeDescriptor, Volume, SHAPE_DESCRIPTOR_TO_VOXEL_INDEX_MAP},
        VoxelDescriptor,
    },
};
use bevy::prelude::UVec3;
use ndshape::Shape as NdShape;

use super::{Materializator, Terrain};

pub struct DefaultMaterializator;

impl Materializator for DefaultMaterializator {
    fn materialize(&self, chunk: &Terrain) -> Grid {
        // todo: this is bof
        let extended_size = CHUNK_SIZE + (UVec3::ONE * 2);
        let grid_shape = crate::world::chunk::Shape {};

        let mut data: Vec<Option<VoxelDescriptor>> =
            vec![None; (extended_size.x * extended_size.y * extended_size.z) as usize];

        for x in 0..CHUNK_SIZE.x {
            for z in 0..CHUNK_SIZE.x {
                let mut depth = 0;

                // reverse iterator to iterate from the surface first
                for y in (0..CHUNK_SIZE.y).rev() {
                    let idx = chunk.shape.linearize([x as u32, y as u32, z as u32]);
                    let shape = chunk.data[idx as usize];
                    let shape_descriptor: ShapeDescriptor = shape.into();
                    depth = match shape.volume {
                        Volume::ZeroSixth => 0,
                        _ => {
                            let voxel_index =
                                SHAPE_DESCRIPTOR_TO_VOXEL_INDEX_MAP[shape_descriptor.0 as usize];
                            if (voxel_index & 0b1111_0000).count_ones()
                                < (voxel_index & 0b0000_1111).count_ones()
                            {
                                0
                            } else {
                                depth + 1
                            }
                        }
                    };

                    let material = if depth <= 3 {
                        material::GRASS
                    } else if depth <= 8 {
                        material::DIRT
                    } else {
                        material::STONE
                    };

                    let block = VoxelDescriptor { shape, material };
                    data[idx as usize] = Some(block);
                }
            }
        }
        Grid {
            size: CHUNK_SIZE,
            // extended_size: CHUNK_SIZE + (UVec3::ONE * 2),
            voxels: data,
            shape: grid_shape,
        }
    }
}
