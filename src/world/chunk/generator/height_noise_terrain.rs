use std::intrinsics::unlikely;

use bevy::prelude::IVec3;
use ndshape::Shape as NdShape;
use noise::{NoiseFn, OpenSimplex};

use crate::world::{
    chunk::{CHUNK_HEIGHT, CHUNK_LENGTH},
    voxel::shape::{Shape, Volume, VOXEL_INDEX_TO_SHAPE_MAP},
};

use super::{Grid, TerrainGenerator};

const WORLD_HEIGHT: f64 = CHUNK_HEIGHT as f64 / 1.1;
const VALUES_LENGTH: u32 = CHUNK_LENGTH + 1;
// const VALUES_HEIGHT: u32 = CHUNK_HEIGHT + 1;

type MapShape = ndshape::ConstShape2u32<VALUES_LENGTH, VALUES_LENGTH>;

pub struct HeightNoiseTerrainGenerator {
    origin: IVec3,
    noise_map: Vec<f32>,
    noise_map_shape: MapShape,
}

impl HeightNoiseTerrainGenerator {
    #[allow(dead_code)]
    pub fn new(origin: IVec3) -> Self {
        let noise = OpenSimplex::default();
        let div = 100.0;

        let noise_map_shape = MapShape {};
        let noise_map: Vec<f32> = (0..noise_map_shape.size())
            .map(|i| {
                let [x, y] = noise_map_shape.delinearize(i);
                noise.get([
                    ((x as i32 + origin.x) as f64) / div,
                    ((y as i32 + origin.z) as f64) / div,
                ]) as f32
            })
            .collect();
        HeightNoiseTerrainGenerator {
            origin,
            noise_map,
            noise_map_shape,
        }
    }
}

impl TerrainGenerator for HeightNoiseTerrainGenerator {
    fn generate(&self, shape: crate::world::chunk::Shape) -> Grid {
        let data = (0..shape.size())
            .map(|i| {
                let [x, y, z] = shape.delinearize(i);

                let idx_0 = self.noise_map[self.noise_map_shape.linearize([x, z]) as usize];
                let idx_1 = self.noise_map[self.noise_map_shape.linearize([x + 1, z]) as usize];
                let idx_2 = self.noise_map[self.noise_map_shape.linearize([x, z + 1]) as usize];
                let idx_3 = self.noise_map[self.noise_map_shape.linearize([x + 1, z + 1]) as usize];

                let idx_0 = ((idx_0 + 1.0) / 2.0 * WORLD_HEIGHT as f32) as u32;
                let idx_1 = ((idx_1 + 1.0) / 2.0 * WORLD_HEIGHT as f32) as u32;
                let idx_2 = ((idx_2 + 1.0) / 2.0 * WORLD_HEIGHT as f32) as u32;
                let idx_3 = ((idx_3 + 1.0) / 2.0 * WORLD_HEIGHT as f32) as u32;

                let index = Self::voxel_idx(&[
                    self.origin.y as u32 + y < idx_0,
                    self.origin.y as u32 + y < idx_1,
                    self.origin.y as u32 + y < idx_2,
                    self.origin.y as u32 + y < idx_3,
                    self.origin.y as u32 + y + 1 < idx_0,
                    self.origin.y as u32 + y + 1 < idx_1,
                    self.origin.y as u32 + y + 1 < idx_2,
                    self.origin.y as u32 + y + 1 < idx_3,
                ]);

                // Fill invalid voxels with empty or full voxels depending on the index
                let mut shape = VOXEL_INDEX_TO_SHAPE_MAP[index as usize];
                if unlikely(shape.volume == Volume::ZeroSixth && index > 0) {
                    shape = if index.count_ones() > 4 {
                        Shape::FULL
                    } else {
                        Shape::EMPTY
                    };
                }
                shape
            })
            .collect();
        Grid { shape, data }
    }
}
