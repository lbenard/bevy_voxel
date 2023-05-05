use bevy::prelude::{IVec3, UVec3};
use interpolation::lerp;
use noise::{NoiseFn, OpenSimplex};

use crate::terrain::{
    block::{Volume, BLOCK_INDEX_TO_SHAPE_MAP},
    chunk::Grid,
    generator::ChunkGenerator,
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

// impl ChunkGenerator for NoiseTerrain {
//     fn generate(&self, chunk: &mut Grid) {
//         for x in 0..=chunk.size.x {
//             for z in 0..=chunk.size.z {
//                 let height = (self.noise.get([(x as f64) / 16.0, (z as f64) / 16.0]) + 1.0) / 2.0;
//                 for y in 0..(2.0 + height * 16.0) as u32 {
//                     let idx = chunk.pos_idx(UVec3 { x, y, z });
//                     chunk.blocks[idx] = true;
//                 }
//             }
//         }
//     }
// }
impl ChunkGenerator for NoiseTerrain {
    fn generate(&self, origin: IVec3, chunk: &mut Grid) {
        for x in 0..chunk.size.x {
            for y in 0..chunk.size.y {
                for z in 0..chunk.size.z {
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
                    let shape = &BLOCK_INDEX_TO_SHAPE_MAP[index as usize];
                    if shape.volume == Volume::ZeroSixth && index > 0 {
                        index = if index.count_ones() > 4 { 255 } else { 0 };
                    }
                    chunk.blocks[grid_index] = index;
                }
            }
        }
    }
}
