use bevy::prelude::IVec3;
use interpolation::lerp;
use ndshape::Shape as NdShape;
use noise::{Cache, NoiseFn, SuperSimplex};

use crate::world::voxel::shape::{Shape, Volume, VOXEL_INDEX_TO_SHAPE_MAP};

use super::{Terrain, TerrainGenerator};

const WORLD_HEIGHT: f64 = 64.0;

pub struct NoiseTerrainGenerator {
    noise: Cache<SuperSimplex>,
}

impl NoiseTerrainGenerator {
    pub fn new() -> Self {
        NoiseTerrainGenerator {
            noise: Cache::<SuperSimplex>::new(SuperSimplex::new(0)),
        }
    }
}

impl TerrainGenerator for NoiseTerrainGenerator {
    fn generate(&self, origin: IVec3, shape: crate::world::chunk::Shape) -> Terrain {
        // arbitrary scale
        let div = 100.0;

        let data = (0..shape.size())
            .map(|i| {
                let [x, y, z] = shape.delinearize(i);

                let lower_height_treshold =
                    lerp(&-1.0, &1.0, &((origin.y + y as i32) as f64 / WORLD_HEIGHT));
                let higher_height_treshold = lerp(
                    &-1.0,
                    &1.0,
                    &((origin.y + y as i32 + 1) as f64 / WORLD_HEIGHT),
                );
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
                let index = Self::voxel_idx(&[
                    _0 > lower_height_treshold,
                    _1 > lower_height_treshold,
                    _2 > lower_height_treshold,
                    _3 > lower_height_treshold,
                    _4 > higher_height_treshold,
                    _5 > higher_height_treshold,
                    _6 > higher_height_treshold,
                    _7 > higher_height_treshold,
                ]);

                // Fill invalid voxels with empty or full voxels depending on the index
                let mut shape = VOXEL_INDEX_TO_SHAPE_MAP[index as usize];
                if shape.volume == Volume::ZeroSixth && index > 0 {
                    shape = if index.count_ones() > 4 {
                        Shape::FULL
                    } else {
                        Shape::EMPTY
                    };
                }
                shape
            })
            .collect();
        Terrain { shape, data }
    }
}
