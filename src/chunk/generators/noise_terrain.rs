use bevy::prelude::{IVec3, UVec3};
use interpolation::lerp;
use noise::{NoiseFn, OpenSimplex};

use crate::{
    chunk::Grid,
    world::terrain::{
        block_descriptor::{
            material::{DIRT, GRASS, STONE},
            shape::{Shape, Volume, BLOCK_INDEX_TO_SHAPE_MAP},
            BlockDescriptor,
        },
        generator::ChunkGenerator,
    },
};

const WORLD_HEIGHT: f64 = 128.0;

pub struct NoiseTerrain {
    noise: OpenSimplex,
}

impl NoiseTerrain {
    pub fn new() -> Self {
        NoiseTerrain {
            noise: OpenSimplex::default(),
        }
    }
}

impl ChunkGenerator for NoiseTerrain {
    fn generate(&self, origin: IVec3, chunk: &mut Grid) {
        for x in -1..chunk.size.x as i32 + 1 {
            for z in -1..chunk.size.z as i32 + 1 {
                let mut depth = 0;
                let div = 50.0;

                for y in (-1..chunk.size.y as i32 + 1).rev() {
                    let lower_height_treshold =
                        lerp(&-1.0, &1.0, &((origin.y + y) as f64 / WORLD_HEIGHT));
                    let higher_height_treshold =
                        lerp(&-1.0, &1.0, &((origin.y + y + 1) as f64 / WORLD_HEIGHT));
                    let _0 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / div,
                        ((y as i32 + origin.y) as f64) / div,
                        ((z as i32 + origin.z) as f64) / div,
                    ]);
                    let _1 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / div,
                        ((y as i32 + origin.y) as f64) / div,
                        ((z as i32 + origin.z) as f64) / div,
                    ]);
                    let _2 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / div,
                        ((y as i32 + origin.y) as f64) / div,
                        ((z as i32 + origin.z + 1) as f64) / div,
                    ]);
                    let _3 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / div,
                        ((y as i32 + origin.y) as f64) / div,
                        ((z as i32 + origin.z + 1) as f64) / div,
                    ]);
                    let _4 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / div,
                        ((y as i32 + origin.y + 1) as f64) / div,
                        ((z as i32 + origin.z) as f64) / div,
                    ]);
                    let _5 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / div,
                        ((y as i32 + origin.y + 1) as f64) / div,
                        ((z as i32 + origin.z) as f64) / div,
                    ]);
                    let _6 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / div,
                        ((y as i32 + origin.y + 1) as f64) / div,
                        ((z as i32 + origin.z + 1) as f64) / div,
                    ]);
                    let _7 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / div,
                        ((y as i32 + origin.y + 1) as f64) / div,
                        ((z as i32 + origin.z + 1) as f64) / div,
                    ]);
                    let index = Self::block_idx(&[
                        _0 > lower_height_treshold,
                        _1 > lower_height_treshold,
                        _2 > lower_height_treshold,
                        _3 > lower_height_treshold,
                        _4 > higher_height_treshold,
                        _5 > higher_height_treshold,
                        _6 > higher_height_treshold,
                        _7 > higher_height_treshold,
                    ]);
                    let grid_index = chunk.pos_idx(UVec3 {
                        x: (x + 1) as u32,
                        y: (y + 1) as u32,
                        z: (z + 1) as u32,
                    });

                    // Fill invalid blocks with empty or full blocks depending on the index
                    let mut shape = BLOCK_INDEX_TO_SHAPE_MAP[index as usize];
                    if shape.volume == Volume::ZeroSixth && index > 0 {
                        shape = if index.count_ones() > 4 {
                            Shape::FULL
                        } else {
                            Shape::EMPTY
                        };
                    }
                    let block = if shape.volume == Volume::ZeroSixth {
                        depth = 0;
                        None
                    } else {
                        depth += 1;
                        let material = if depth <= 3 {
                            GRASS
                        } else if depth <= 8 {
                            DIRT
                        } else {
                            STONE
                        };
                        Some(BlockDescriptor { shape, material })
                    };
                    chunk.blocks[grid_index] = block;
                }
            }
        }
    }
}
