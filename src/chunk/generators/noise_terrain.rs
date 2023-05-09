use bevy::prelude::{IVec3, UVec3};
use interpolation::lerp;
use noise::{NoiseFn, OpenSimplex};

use crate::{
    chunk::Grid,
    terrain::{
        block::{
            material::{Material, DIRT, GRASS, STONE},
            shape::{Shape, Volume, BLOCK_INDEX_TO_SHAPE_MAP},
            Block,
        },
        generator::ChunkGenerator,
    },
};

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
        for x in 0..chunk.size.x {
            for z in 0..chunk.size.z {
                let mut top_block: Option<Block> = None;

                for y in (0..chunk.size.y).rev() {
                    let lower_height_treshold = lerp(
                        &-1.0,
                        &1.0,
                        &((y as i32 + origin.y) as f64 / chunk.size.y as f64),
                    );
                    let higher_height_treshold = lerp(
                        &-1.0,
                        &1.0,
                        &((y as i32 + origin.y + 1) as f64 / chunk.size.y as f64),
                    );
                    let _0 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / 32.0,
                        ((y as i32 + origin.y) as f64) / 32.0,
                        ((z as i32 + origin.z) as f64) / 32.0,
                    ]);
                    let _1 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / 32.0,
                        ((y as i32 + origin.y) as f64) / 32.0,
                        ((z as i32 + origin.z) as f64) / 32.0,
                    ]);
                    let _2 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / 32.0,
                        ((y as i32 + origin.y) as f64) / 32.0,
                        ((z as i32 + origin.z + 1) as f64) / 32.0,
                    ]);
                    let _3 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / 32.0,
                        ((y as i32 + origin.y) as f64) / 32.0,
                        ((z as i32 + origin.z + 1) as f64) / 32.0,
                    ]);
                    let _4 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / 32.0,
                        ((y as i32 + origin.y + 1) as f64) / 32.0,
                        ((z as i32 + origin.z) as f64) / 32.0,
                    ]);
                    let _5 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / 32.0,
                        ((y as i32 + origin.y + 1) as f64) / 32.0,
                        ((z as i32 + origin.z) as f64) / 32.0,
                    ]);
                    let _6 = self.noise.get([
                        ((x as i32 + origin.x) as f64) / 32.0,
                        ((y as i32 + origin.y + 1) as f64) / 32.0,
                        ((z as i32 + origin.z + 1) as f64) / 32.0,
                    ]);
                    let _7 = self.noise.get([
                        ((x as i32 + origin.x + 1) as f64) / 32.0,
                        ((y as i32 + origin.y + 1) as f64) / 32.0,
                        ((z as i32 + origin.z + 1) as f64) / 32.0,
                    ]);
                    let mut index = Self::block_idx(&[
                        _0 > lower_height_treshold,
                        _1 > lower_height_treshold,
                        _2 > lower_height_treshold,
                        _3 > lower_height_treshold,
                        _4 > higher_height_treshold,
                        _5 > higher_height_treshold,
                        _6 > higher_height_treshold,
                        _7 > higher_height_treshold,
                    ]);
                    let grid_index = chunk.pos_idx(UVec3 { x, y, z });

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
                        None
                    } else {
                        let material = if let Some(top_block) = top_block {
                            if top_block.material == GRASS {
                                DIRT
                            } else if top_block.material == DIRT {
                                STONE
                            } else {
                                STONE
                            }
                        } else {
                            GRASS
                        };
                        Some(Block { shape, material })
                    };
                    chunk.blocks[grid_index] = block;
                    top_block = block;
                }
            }
        }
    }
}
