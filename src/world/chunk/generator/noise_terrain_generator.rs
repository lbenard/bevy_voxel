use std::intrinsics::unlikely;

use bevy::prelude::IVec3;
use interpolation::lerp;
use ndshape::Shape as NdShape;
use noise::{Cache, NoiseFn, SuperSimplex};

use crate::world::{
    chunk::{CHUNK_HEIGHT, CHUNK_LENGTH},
    voxel::shape::{Shape, Volume, VOXEL_INDEX_TO_SHAPE_MAP},
};

use super::{Terrain, TerrainGenerator};

const WORLD_HEIGHT: f64 = CHUNK_HEIGHT as f64 / 1.5;
const VALUES_LENGTH: u32 = CHUNK_LENGTH + 1;
const VALUES_HEIGHT: u32 = CHUNK_HEIGHT + 1;
type ValuesShape = ndshape::ConstShape3u32<VALUES_LENGTH, VALUES_HEIGHT, VALUES_LENGTH>;

pub struct NoiseTerrainGenerator {
    origin: IVec3,
    values: Vec<f32>,
    values_shape: ValuesShape,
}

impl NoiseTerrainGenerator {
    pub fn new(origin: IVec3) -> Self {
        let noise = Cache::<SuperSimplex>::new(SuperSimplex::new(0));
        // arbitrary scale
        let div = 100.0;
        let values_shape = ValuesShape {};

        // TODO: maybe fill by linearizing instead of delinearizing as it cost less
        let values: Vec<f32> = (0..values_shape.size())
            .map(|i| {
                let [x, y, z] = values_shape.delinearize(i);
                noise.get([
                    ((x as i32 + origin.x) as f64) / div,
                    ((y as i32 + origin.y) as f64) / div,
                    ((z as i32 + origin.z) as f64) / div,
                ]) as f32
            })
            .collect();

        NoiseTerrainGenerator {
            origin,
            values,
            values_shape,
        }
    }
}

impl TerrainGenerator for NoiseTerrainGenerator {
    fn generate(&self, shape: crate::world::chunk::Shape) -> Terrain {
        let data = (0..shape.size())
            .map(|i| {
                let [x, y, z] = shape.delinearize(i);

                let lower_height_treshold = lerp(
                    &-1.0,
                    &1.0,
                    &((self.origin.y + y as i32) as f32 / WORLD_HEIGHT as f32),
                );
                let higher_height_treshold = lerp(
                    &-1.0,
                    &1.0,
                    &((self.origin.y + y as i32 + 1) as f32 / WORLD_HEIGHT as f32),
                );
                let _0 = self.values[self.values_shape.linearize([x, y, z]) as usize];
                let _1 = self.values[self.values_shape.linearize([x + 1, y, z]) as usize];
                let _2 = self.values[self.values_shape.linearize([x, y, z + 1]) as usize];
                let _3 = self.values[self.values_shape.linearize([x + 1, y, z + 1]) as usize];
                let _4 = self.values[self.values_shape.linearize([x, y + 1, z]) as usize];
                let _5 = self.values[self.values_shape.linearize([x + 1, y + 1, z]) as usize];
                let _6 = self.values[self.values_shape.linearize([x, y + 1, z + 1]) as usize];
                let _7 = self.values[self.values_shape.linearize([x + 1, y + 1, z + 1]) as usize];
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
        Terrain { shape, data }
    }
}
