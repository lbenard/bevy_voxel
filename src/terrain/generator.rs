use bevy::prelude::IVec3;

use crate::chunk::{BlockIndex, Grid};

pub trait ChunkGenerator {
    fn generate(&self, origin: IVec3, chunk: &mut Grid);

    fn block_idx(bits: &[bool; 8]) -> BlockIndex {
        let mut idx = 0;
        idx |= (bits[0] as u8) << 0;
        idx |= (bits[1] as u8) << 1;
        idx |= (bits[2] as u8) << 2;
        idx |= (bits[3] as u8) << 3;
        idx |= (bits[4] as u8) << 4;
        idx |= (bits[5] as u8) << 5;
        idx |= (bits[6] as u8) << 6;
        idx |= (bits[7] as u8) << 7;
        idx
    }
}
