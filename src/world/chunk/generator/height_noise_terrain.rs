// use bevy::prelude::{IVec3, UVec3};
// use interpolation::lerp;
// use noise::{NoiseFn, OpenSimplex};

// use crate::world::{
//     chunk::Grid,
//     voxel::{
//         material::{DIRT, GRASS, STONE},
//         shape::Shape,
//         VoxelDescriptor,
//     },
// };

// const WORLD_HEIGHT: f64 = 32.0;

// pub struct HeightNoiseTerrain {
//     noise: OpenSimplex,
// }

// impl HeightNoiseTerrain {
//     #[allow(dead_code)]
//     pub fn new() -> Self {
//         HeightNoiseTerrain {
//             noise: OpenSimplex::default(),
//         }
//     }
// }

// impl ChunkGenerator for HeightNoiseTerrain {
//     fn generate(&self, origin: IVec3, chunk: &mut Grid) {
//         for x in -1..chunk.size.x as i32 + 1 {
//             for z in -1..chunk.size.z as i32 + 1 {
//                 let mut depth = 0;
//                 let div = 50.0;
//                 let height = lerp(
//                     &0.0,
//                     &WORLD_HEIGHT,
//                     &(self.noise.get([
//                         ((x as i32 + origin.x) as f64) / div,
//                         ((z as i32 + origin.z) as f64) / div,
//                     ]) / 2.0
//                         + 0.5),
//                 ) as i32;
//                 for y in (-1..height).rev() {
//                     // let lower_height_treshold =
//                     //     lerp(&-1.0, &1.0, &((origin.y + y) as f64 / WORLD_HEIGHT));
//                     // let higher_height_treshold =
//                     //     lerp(&-1.0, &1.0, &((origin.y + y + 1) as f64 / WORLD_HEIGHT));
//                     // let _0 = self.noise.get([
//                     //     ((x as i32 + origin.x) as f64) / div,
//                     //     ((y as i32 + origin.y) as f64) / div,
//                     //     ((z as i32 + origin.z) as f64) / div,
//                     // ]);
//                     // let _1 = self.noise.get([
//                     //     ((x as i32 + origin.x + 1) as f64) / div,
//                     //     ((y as i32 + origin.y) as f64) / div,
//                     //     ((z as i32 + origin.z) as f64) / div,
//                     // ]);
//                     // let _2 = self.noise.get([
//                     //     ((x as i32 + origin.x) as f64) / div,
//                     //     ((y as i32 + origin.y) as f64) / div,
//                     //     ((z as i32 + origin.z + 1) as f64) / div,
//                     // ]);
//                     // let _3 = self.noise.get([
//                     //     ((x as i32 + origin.x + 1) as f64) / div,
//                     //     ((y as i32 + origin.y) as f64) / div,
//                     //     ((z as i32 + origin.z + 1) as f64) / div,
//                     // ]);
//                     // let _4 = self.noise.get([
//                     //     ((x as i32 + origin.x) as f64) / div,
//                     //     ((y as i32 + origin.y + 1) as f64) / div,
//                     //     ((z as i32 + origin.z) as f64) / div,
//                     // ]);
//                     // let _5 = self.noise.get([
//                     //     ((x as i32 + origin.x + 1) as f64) / div,
//                     //     ((y as i32 + origin.y + 1) as f64) / div,
//                     //     ((z as i32 + origin.z) as f64) / div,
//                     // ]);
//                     // let _6 = self.noise.get([
//                     //     ((x as i32 + origin.x) as f64) / div,
//                     //     ((y as i32 + origin.y + 1) as f64) / div,
//                     //     ((z as i32 + origin.z + 1) as f64) / div,
//                     // ]);
//                     // let _7 = self.noise.get([
//                     //     ((x as i32 + origin.x + 1) as f64) / div,
//                     //     ((y as i32 + origin.y + 1) as f64) / div,
//                     //     ((z as i32 + origin.z + 1) as f64) / div,
//                     // ]);
//                     // let index = Self::voxel_idx(&[
//                     //     _0 > lower_height_treshold,
//                     //     _1 > lower_height_treshold,
//                     //     _2 > lower_height_treshold,
//                     //     _3 > lower_height_treshold,
//                     //     _4 > higher_height_treshold,
//                     //     _5 > higher_height_treshold,
//                     //     _6 > higher_height_treshold,
//                     //     _7 > higher_height_treshold,
//                     // ]);
//                     let grid_index = chunk.pos_idx(UVec3 {
//                         x: (x + 1) as u32,
//                         y: (y + 1) as u32,
//                         z: (z + 1) as u32,
//                     });

//                     // Fill invalid voxels with empty or full voxels depending on the index
//                     let shape = Shape::FULL;
//                     depth += 1;
//                     let material = if depth <= 3 {
//                         GRASS
//                     } else if depth <= 8 {
//                         DIRT
//                     } else {
//                         STONE
//                     };
//                     let voxel = Some(VoxelDescriptor { shape, material });
//                     chunk.voxels[grid_index] = voxel;
//                 }
//             }
//         }
//     }
// }
