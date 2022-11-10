use bevy::prelude::UVec3;

use crate::terrain::{chunk::Grid, generator::ChunkGenerator};

pub struct FlatTerrain;

impl FlatTerrain {
    pub fn new() -> Self {
        FlatTerrain {}
    }
}

impl ChunkGenerator for FlatTerrain {
    fn generate(&self, chunk: &mut Grid) {
        for x in 0..chunk.size.x {
            for z in 0..chunk.size.z {
                for y in 0..x + 1 {
                    let idx = chunk.pos_idx(UVec3 { x, y, z });
                    chunk.blocks[idx] = 0b1111_1111;
                    if y == x {
                        chunk.blocks[idx] = 0b0110_1111;
                    }
                }
            }
        }
    }
}
