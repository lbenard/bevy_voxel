use bevy::prelude::{IVec3, UVec3};

use crate::{chunk::Grid, terrain::generator::ChunkGenerator};

pub struct FlatTerrain;

impl FlatTerrain {
    pub fn new() -> Self {
        FlatTerrain {}
    }
}

impl ChunkGenerator for FlatTerrain {
    fn generate(&self, _origin: IVec3, chunk: &mut Grid) {
        for x in 0..chunk.size.x {
            for z in 0..chunk.size.z {
                for y in 0..x + 1 {
                    let idx = chunk.pos_idx(UVec3 { x, y, z });
                    chunk.blocks[idx] = 0b1111_1111;
                }
            }
        }
    }
}
