use std::sync::LazyLock;

use bevy::prelude::{EulerRot, Mat4, UVec3, Vec3};

use crate::terrain::block::Volume;

use super::block::{
    vertex_to_index, Shape, FIVE_SIXTH_VERTEX_LIST, FOUR_SIXTH_VERTEX_LIST, ONE_SIXTH_VERTEX_LIST,
    SIX_SIXTH_VERTEX_LIST, THREE_SIXTH_VERTEX_LIST, TWO_SIXTH_VERTEX_LIST, ZERO_SIXTH_VERTEX_LIST,
};

// Towards +y
pub const TOP_FACE_MASK: u8 = 0b1111_0000;
// pub const VALID_TOP_FACES: [u8; 6] = [
//     0b0000_0000,
//     0b1111_0000,
//     0b0111_0000,
//     0b1011_0000,
//     0b1101_0000,
//     0b1110_0000,
// ];

// Towards -y
pub const BOTTOM_FACE_MASK: u8 = 0b0000_1111;
// pub const VALID_BOTTOM_FACES: [u8; 6] = [
//     0b0000_0000,
//     0b0000_1111,
//     0b0000_0111,
//     0b0000_1011,
//     0b0000_1101,
//     0b0000_1110,
// ];

// Towards +x
pub const WEST_FACE_MASK: u8 = 0b1010_1010;
// pub const VALID_WEST_FACES: [u8; 6] = [
//     0b0000_0000,
//     0b0110_0110,
//     0b0010_0110,
//     0b0100_0110,
//     0b0110_0010,
//     0b0110_0100,
// ];

// Towards +z
pub const NORTH_FACE_MASK: u8 = 0b1100_1100;
// pub const VALID_NORTH_FACES: [u8; 6] = [
//     0b0000_0000,
//     0b1100_1100,
//     0b0100_1100,
//     0b1000_1100,
//     0b1100_0100,
//     0b1100_1000,
// ];

// Towards -x
pub const EAST_FACE_MASK: u8 = 0b0101_0101;
// pub const VALID_EAST_FACES: [u8; 6] = [
//     0b0000_0000,
//     0b1001_1001,
//     0b0001_1001,
//     0b1000_1001,
//     0b1001_0001,
//     0b1001_1000,
// ];

// Towards -z
pub const SOUTH_FACE_MASK: u8 = 0b0011_0011;
// pub const VALID_SOUTH_FACES: [u8; 6] = [
//     0b0000_0000,
//     0b0011_0011,
//     0b0001_0011,
//     0b0010_0011,
//     0b0011_0001,
//     0b0011_0010,
// ];

pub static BLOCK_INDEX_TO_SHAPE_MAP: LazyLock<[Shape; 256]> = LazyLock::new(|| {
    let mut map: [Shape; 256] = [Shape::EMPTY; 256];
    let facing_rotations = [
        Vec3::new(0.0, 0.0, 0.0),                     // North
        Vec3::new(0.0, -90.0_f32.to_radians(), 0.0),  // East
        Vec3::new(0.0, -180.0_f32.to_radians(), 0.0), // South
        Vec3::new(0.0, -270.0_f32.to_radians(), 0.0), // West
        Vec3::new(0.0, 0.0, 90.0_f32.to_radians()),   // Top
        Vec3::new(0.0, 0.0, -90.0_f32.to_radians()),  // Bottom
    ];
    // Angles are negative as the angle describes the angle seen when facing the cube from the outside, not the inside
    let face_rotations = [
        Vec3::new(0.0, 0.0, 0.0),                     // 0 degrees
        Vec3::new(-90.0_f32.to_radians(), 0.0, 0.0),  // 90 degrees
        Vec3::new(-180.0_f32.to_radians(), 0.0, 0.0), // 180 degrees
        Vec3::new(-270.0_f32.to_radians(), 0.0, 0.0), // 270 degrees
    ];

    for (vertex_list_index, vertex_list) in [
        &ZERO_SIXTH_VERTEX_LIST,
        &ONE_SIXTH_VERTEX_LIST,
        &TWO_SIXTH_VERTEX_LIST,
        &THREE_SIXTH_VERTEX_LIST,
        &FOUR_SIXTH_VERTEX_LIST,
        &FIVE_SIXTH_VERTEX_LIST,
        &SIX_SIXTH_VERTEX_LIST,
    ]
    .iter()
    .enumerate()
    {
        for (facing_rotation_index, facing_rotation) in facing_rotations.iter().enumerate() {
            for (face_rotation_index, face_rotation) in face_rotations.iter().enumerate() {
                let rotation = *facing_rotation + *face_rotation;
                let rot = Mat4::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
                let rotated_vertices = vertex_list
                    .iter()
                    .map(|vertex| {
                        let center_at_origin = vertex.as_vec3() - Vec3::new(0.5, 0.5, 0.5);
                        let rotated = rot.transform_vector3(center_at_origin);
                        (rotated + Vec3::new(0.5, 0.5, 0.5)).round().as_uvec3()
                    })
                    .collect::<Vec<UVec3>>();
                let grid_index = rotated_vertices
                    .iter()
                    .fold(0, |acc, vertex| acc | (1 << vertex_to_index(*vertex)));

                // Use more aesthetically pleasing shapes for natural generation. The 2/6 and 4/6 both look weird for slopes
                let aesthetic_volume_index = match vertex_list_index {
                    2 => 1,
                    4 => 5,
                    x => x,
                };

                if map[grid_index].volume == Volume::ZeroSixth {
                    map[grid_index] = Shape::new(
                        ((facing_rotation_index * 4 + face_rotation_index) as u8)
                            .try_into()
                            .unwrap(),
                        (aesthetic_volume_index as u8).try_into().unwrap(),
                    )
                }
            }
        }
    }

    map
});

// pub(super) const INDEX_LOOKUP: LazyLock<[Vec<[UVec3; 3]>; 256]> = LazyLock::new(|| {
// pub const INDEX_LOOKUP: LazyLock<[Vec<[UVec3; 3]>; 256]> = LazyLock::new(|| {
//     [
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 1, y: 0, z: 0 },
//         ]],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//         ]],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 1, y: 1, z: 1 },
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 0, y: 0, z: 1 },
//         ]],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 1, y: 1, z: 1 },
//         ]],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 1, y: 1, z: 1 },
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 0, y: 0, z: 1 },
//         ]],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//         ]],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//         ]],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 0, z: 1 },
//         ]],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 1, y: 1, z: 1 },
//         ]],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 1, y: 1, z: 1 },
//             UVec3 { x: 1, y: 0, z: 0 },
//         ]],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//         ]],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//         ],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 0, z: 0 },
//         ]],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 0 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//             ],
//         ],
//         vec![],
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//             ],
//         ],
//         vec![[
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//         ]],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//         ],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//         ],
//         vec![],
//         vec![[
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 1, y: 1, z: 1 },
//         ]],
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//         ],
//         vec![[
//             UVec3 { x: 1, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 1, z: 0 },
//         ]],
//         vec![[
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 1, y: 0, z: 0 },
//         ]],
//         vec![],
//     ]
// });

// pub const Y_EXTERIOR_FACE_LOOKUP: LazyLock<[Vec<[UVec3; 3]>; 256]> = LazyLock::new(|| {
//     [
//         vec![], // 0
//         vec![[
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 0, y: 0, z: 1 },
//         ]], // 1
//         vec![[
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//         ]], // 2
//         vec![], // 3
//         vec![[
//             UVec3 { x: 1, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 1, y: 0, z: 0 },
//         ]], // 4
//         vec![], // 5
//         vec![], // 6
//         vec![[
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//         ]], // 7
//         vec![[
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//         ]], // 8
//         vec![], // 9
//         vec![], // 10
//         vec![[
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 0 },
//             UVec3 { x: 0, y: 0, z: 1 },
//         ]], // 11
//         vec![], // 12
//         vec![[
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 0 },
//             UVec3 { x: 1, y: 0, z: 1 },
//         ]], // 13
//         vec![[
//             UVec3 { x: 1, y: 0, z: 1 },
//             UVec3 { x: 0, y: 0, z: 1 },
//             UVec3 { x: 1, y: 0, z: 0 },
//         ]], // 14
//         vec![
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//                 UVec3 { x: 0, y: 0, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 0 },
//                 UVec3 { x: 1, y: 0, z: 1 },
//             ],
//         ], // 15
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 0 },
//         ]], // 16
//         vec![], // 17
//         vec![], // 18
//         vec![], // 19
//         vec![], // 20
//         vec![], // 21
//         vec![], // 22
//         vec![], // 23
//         vec![], // 24
//         vec![], // 25
//         vec![], // 26
//         vec![], // 27
//         vec![], // 28
//         vec![], // 29
//         vec![], // 30
//         vec![], // 31
//         vec![[
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 1, y: 1, z: 1 },
//         ]], // 32
//         vec![], // 33
//         vec![], // 34
//         vec![], // 35
//         vec![], // 36
//         vec![], // 37
//         vec![], // 38
//         vec![], // 39
//         vec![], // 40
//         vec![], // 41
//         vec![], // 42
//         vec![], // 43
//         vec![], // 44
//         vec![], // 45
//         vec![], // 46
//         vec![], // 47
//         vec![], // 48
//         vec![], // 49
//         vec![], // 50
//         vec![], // 51
//         vec![], // 52
//         vec![], // 53
//         vec![], // 54
//         vec![], // 55
//         vec![], // 56
//         vec![], // 57
//         vec![], // 58
//         vec![], // 59
//         vec![], // 60
//         vec![], // 61
//         vec![], // 62
//         vec![], // 63
//         vec![[
//             UVec3 { x: 1, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//         ]], // 64
//         vec![], // 65
//         vec![], // 66
//         vec![], // 67
//         vec![], // 68
//         vec![], // 69
//         vec![], // 70
//         vec![], // 71
//         vec![], // 72
//         vec![], // 73
//         vec![], // 74
//         vec![], // 75
//         vec![], // 76
//         vec![], // 77
//         vec![], // 78
//         vec![], // 79
//         vec![], // 80
//         vec![], // 81
//         vec![], // 82
//         vec![], // 83
//         vec![], // 84
//         vec![], // 85
//         vec![], // 86
//         vec![], // 87
//         vec![], // 88
//         vec![], // 89
//         vec![], // 90
//         vec![], // 91
//         vec![], // 92
//         vec![], // 93
//         vec![], // 94
//         vec![], // 95
//         vec![], // 96
//         vec![], // 97
//         vec![], // 98
//         vec![], // 99
//         vec![], // 100
//         vec![], // 101
//         vec![], // 102
//         vec![], // 103
//         vec![], // 104
//         vec![], // 105
//         vec![], // 106
//         vec![], // 107
//         vec![], // 108
//         vec![], // 109
//         vec![], // 110
//         vec![], // 111
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 1, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 0 },
//         ]], // 112
//         vec![], // 113
//         vec![], // 114
//         vec![], // 115
//         vec![], // 116
//         vec![], // 117
//         vec![], // 118
//         vec![], // 119
//         vec![], // 120
//         vec![], // 121
//         vec![], // 122
//         vec![], // 123
//         vec![], // 124
//         vec![], // 125
//         vec![], // 126
//         vec![], // 127
//         vec![[
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 1 },
//             UVec3 { x: 0, y: 1, z: 0 },
//         ]], // 128
//         vec![], // 129
//         vec![], // 130
//         vec![], // 131
//         vec![], // 132
//         vec![], // 133
//         vec![], // 134
//         vec![], // 135
//         vec![], // 136
//         vec![], // 137
//         vec![], // 138
//         vec![], // 139
//         vec![], // 140
//         vec![], // 141
//         vec![], // 142
//         vec![], // 143
//         vec![], // 144
//         vec![], // 145
//         vec![], // 146
//         vec![], // 147
//         vec![], // 148
//         vec![], // 149
//         vec![], // 150
//         vec![], // 151
//         vec![], // 152
//         vec![], // 153
//         vec![], // 154
//         vec![], // 155
//         vec![], // 156
//         vec![], // 157
//         vec![], // 158
//         vec![], // 159
//         vec![], // 160
//         vec![], // 161
//         vec![], // 162
//         vec![], // 163
//         vec![], // 164
//         vec![], // 165
//         vec![], // 166
//         vec![], // 167
//         vec![], // 168
//         vec![], // 169
//         vec![], // 170
//         vec![], // 171
//         vec![], // 172
//         vec![], // 173
//         vec![], // 174
//         vec![], // 175
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 0 },
//         ]], // 176
//         vec![], // 177
//         vec![], // 178
//         vec![], // 179
//         vec![], // 180
//         vec![], // 181
//         vec![], // 182
//         vec![], // 183
//         vec![], // 184
//         vec![], // 185
//         vec![], // 186
//         vec![], // 187
//         vec![], // 188
//         vec![], // 189
//         vec![], // 190
//         vec![], // 191
//         vec![], // 192
//         vec![], // 193
//         vec![], // 194
//         vec![], // 195
//         vec![], // 196
//         vec![], // 197
//         vec![], // 198
//         vec![], // 199
//         vec![], // 200
//         vec![], // 201
//         vec![], // 202
//         vec![], // 203
//         vec![], // 204
//         vec![], // 205
//         vec![], // 206
//         vec![], // 207
//         vec![[
//             UVec3 { x: 0, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 1 },
//         ]], // 208
//         vec![], // 209
//         vec![], // 210
//         vec![], // 211
//         vec![], // 212
//         vec![], // 213
//         vec![], // 214
//         vec![], // 215
//         vec![], // 216
//         vec![], // 217
//         vec![], // 218
//         vec![], // 219
//         vec![], // 220
//         vec![], // 221
//         vec![], // 222
//         vec![], // 223
//         vec![[
//             UVec3 { x: 1, y: 1, z: 0 },
//             UVec3 { x: 0, y: 1, z: 1 },
//             UVec3 { x: 1, y: 1, z: 1 },
//         ]], // 224
//         vec![], // 225
//         vec![], // 226
//         vec![], // 227
//         vec![], // 228
//         vec![], // 229
//         vec![], // 230
//         vec![], // 231
//         vec![], // 232
//         vec![], // 233
//         vec![], // 234
//         vec![], // 235
//         vec![], // 236
//         vec![], // 237
//         vec![], // 238
//         vec![], // 239
//         vec![
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 0, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//             ],
//             [
//                 UVec3 { x: 0, y: 1, z: 0 },
//                 UVec3 { x: 1, y: 1, z: 1 },
//                 UVec3 { x: 1, y: 1, z: 0 },
//             ],
//         ], // 240
//         vec![], // 241
//         vec![], // 242
//         vec![], // 243
//         vec![], // 244
//         vec![], // 245
//         vec![], // 246
//         vec![], // 247
//         vec![], // 248
//         vec![], // 249
//         vec![], // 250
//         vec![], // 251
//         vec![], // 252
//         vec![], // 253
//         vec![], // 254
//         vec![], // 255
//     ]
// });

// pub const Z_EXTERIOR_FACE_LOOKUP: LazyLock<[Vec<[UVec3; 3]>; 256]> = LazyLock::new(|| {
//     [
//         vec![], // 0
//         vec![], // 1
//         vec![], // 2
//         vec![], // 3
//         vec![], // 4
//         vec![], // 5
//         vec![], // 6
//         vec![], // 7
//         vec![], // 8
//         vec![], // 9
//         vec![], // 10
//         vec![], // 11
//         vec![], // 12
//         vec![], // 13
//         vec![], // 14
//         vec![], // 15
//         vec![], // 16
//         vec![], // 17
//         vec![], // 18
//         vec![], // 19
//         vec![], // 20
//         vec![], // 21
//         vec![], // 22
//         vec![], // 23
//         vec![], // 24
//         vec![], // 25
//         vec![], // 26
//         vec![], // 27
//         vec![], // 28
//         vec![], // 29
//         vec![], // 30
//         vec![], // 31
//         vec![], // 32
//         vec![], // 33
//         vec![], // 34
//         vec![], // 35
//         vec![], // 36
//         vec![], // 37
//         vec![], // 38
//         vec![], // 39
//         vec![], // 40
//         vec![], // 41
//         vec![], // 42
//         vec![], // 43
//         vec![], // 44
//         vec![], // 45
//         vec![], // 46
//         vec![], // 47
//         vec![], // 48
//         vec![], // 49
//         vec![], // 50
//         vec![], // 51
//         vec![], // 52
//         vec![], // 53
//         vec![], // 54
//         vec![], // 55
//         vec![], // 56
//         vec![], // 57
//         vec![], // 58
//         vec![], // 59
//         vec![], // 60
//         vec![], // 61
//         vec![], // 62
//         vec![], // 63
//         vec![], // 64
//         vec![], // 65
//         vec![], // 66
//         vec![], // 67
//         vec![], // 68
//         vec![], // 69
//         vec![], // 70
//         vec![], // 71
//         vec![], // 72
//         vec![], // 73
//         vec![], // 74
//         vec![], // 75
//         vec![], // 76
//         vec![], // 77
//         vec![], // 78
//         vec![], // 79
//         vec![], // 80
//         vec![], // 81
//         vec![], // 82
//         vec![], // 83
//         vec![], // 84
//         vec![], // 85
//         vec![], // 86
//         vec![], // 87
//         vec![], // 88
//         vec![], // 89
//         vec![], // 90
//         vec![], // 91
//         vec![], // 92
//         vec![], // 93
//         vec![], // 94
//         vec![], // 95
//         vec![], // 96
//         vec![], // 97
//         vec![], // 98
//         vec![], // 99
//         vec![], // 100
//         vec![], // 101
//         vec![], // 102
//         vec![], // 103
//         vec![], // 104
//         vec![], // 105
//         vec![], // 106
//         vec![], // 107
//         vec![], // 108
//         vec![], // 109
//         vec![], // 110
//         vec![], // 111
//         vec![], // 112
//         vec![], // 113
//         vec![], // 114
//         vec![], // 115
//         vec![], // 116
//         vec![], // 117
//         vec![], // 118
//         vec![], // 119
//         vec![], // 120
//         vec![], // 121
//         vec![], // 122
//         vec![], // 123
//         vec![], // 124
//         vec![], // 125
//         vec![], // 126
//         vec![], // 127
//         vec![], // 128
//         vec![], // 129
//         vec![], // 130
//         vec![], // 131
//         vec![], // 132
//         vec![], // 133
//         vec![], // 134
//         vec![], // 135
//         vec![], // 136
//         vec![], // 137
//         vec![], // 138
//         vec![], // 139
//         vec![], // 140
//         vec![], // 141
//         vec![], // 142
//         vec![], // 143
//         vec![], // 144
//         vec![], // 145
//         vec![], // 146
//         vec![], // 147
//         vec![], // 148
//         vec![], // 149
//         vec![], // 150
//         vec![], // 151
//         vec![], // 152
//         vec![], // 153
//         vec![], // 154
//         vec![], // 155
//         vec![], // 156
//         vec![], // 157
//         vec![], // 158
//         vec![], // 159
//         vec![], // 160
//         vec![], // 161
//         vec![], // 162
//         vec![], // 163
//         vec![], // 164
//         vec![], // 165
//         vec![], // 166
//         vec![], // 167
//         vec![], // 168
//         vec![], // 169
//         vec![], // 170
//         vec![], // 171
//         vec![], // 172
//         vec![], // 173
//         vec![], // 174
//         vec![], // 175
//         vec![], // 176
//         vec![], // 177
//         vec![], // 178
//         vec![], // 179
//         vec![], // 180
//         vec![], // 181
//         vec![], // 182
//         vec![], // 183
//         vec![], // 184
//         vec![], // 185
//         vec![], // 186
//         vec![], // 187
//         vec![], // 188
//         vec![], // 189
//         vec![], // 190
//         vec![], // 191
//         vec![], // 192
//         vec![], // 193
//         vec![], // 194
//         vec![], // 195
//         vec![], // 196
//         vec![], // 197
//         vec![], // 198
//         vec![], // 199
//         vec![], // 200
//         vec![], // 201
//         vec![], // 202
//         vec![], // 203
//         vec![], // 204
//         vec![], // 205
//         vec![], // 206
//         vec![], // 207
//         vec![], // 208
//         vec![], // 209
//         vec![], // 210
//         vec![], // 211
//         vec![], // 212
//         vec![], // 213
//         vec![], // 214
//         vec![], // 215
//         vec![], // 216
//         vec![], // 217
//         vec![], // 218
//         vec![], // 219
//         vec![], // 220
//         vec![], // 221
//         vec![], // 222
//         vec![], // 223
//         vec![], // 224
//         vec![], // 225
//         vec![], // 226
//         vec![], // 227
//         vec![], // 228
//         vec![], // 229
//         vec![], // 230
//         vec![], // 231
//         vec![], // 232
//         vec![], // 233
//         vec![], // 234
//         vec![], // 235
//         vec![], // 236
//         vec![], // 237
//         vec![], // 238
//         vec![], // 239
//         vec![], // 240
//         vec![], // 241
//         vec![], // 242
//         vec![], // 243
//         vec![], // 244
//         vec![], // 245
//         vec![], // 246
//         vec![], // 247
//         vec![], // 248
//         vec![], // 249
//         vec![], // 250
//         vec![], // 251
//         vec![], // 252
//         vec![], // 253
//         vec![], // 254
//         vec![], // 255
//     ]
// });
