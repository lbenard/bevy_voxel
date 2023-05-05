use std::sync::LazyLock;

use bevy::prelude::{EulerRot, Mat4, UVec3, Vec3};

use super::chunk::BlockIndex;

type BlockDescriptor = u8;
type GridIndex = u8;

// Towards +y
pub const TOP_FACE_MASK: u8 = 0b1111_0000;

// Towards -y
pub const BOTTOM_FACE_MASK: u8 = 0b0000_1111;

// Towards +x
pub const WEST_FACE_MASK: u8 = 0b1010_1010;

// Towards +z
pub const NORTH_FACE_MASK: u8 = 0b1100_1100;

// Towards -x
pub const EAST_FACE_MASK: u8 = 0b0101_0101;

// Towards -z
pub const SOUTH_FACE_MASK: u8 = 0b0011_0011;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Volume {
    ZeroSixth,
    OneSixth,
    TwoSixth,
    ThreeSixth,
    FourSixth,
    FiveSixth,
    SixSixth,
}

impl TryFrom<u8> for Volume {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        [
            Self::ZeroSixth,
            Self::OneSixth,
            Self::TwoSixth,
            Self::ThreeSixth,
            Self::FourSixth,
            Self::FiveSixth,
            Self::SixSixth,
        ]
        .get(value as usize)
        .ok_or_else(|| ())
        .map(|v| *v)
    }
}

/// Cubes have 24 rotation combinations. One way to visualize it (and the way that this enum represents) is to
/// understand that for all combinations to exist, any given face can be in any of the 4 possible rotations around their
/// own axis, and can face any of the 6 directions (+x, -x, +y, -y, +z and -z).
/// Meaning that there is 4*6 = 24 combinations.
///
/// Individual face rotations cannot have a normalized rotation representation that doesn't depend on which direction
/// it's facing.
/// Consider a cube pattern made of paper.
/// Often, the top and bottom faces are perpendicular to the 4 other faces that represent the side faces.
/// Any cube pattern should have at least 2 perpendicular faces like that.
/// Those 2 faces rotation system won't be the same depending on where they are placed relative to the other 4 faces.
/// Which is why we have to make a choice about said system.
///
/// Once we think about paper cube patterns, the less arbitrary rotation system, once mapped to a cube pattern,
/// would look like this:
///
/// ┌────────┐
/// │        │
/// │  Top   │
/// │        │
/// ├────────┼────────┬────────┬────────┐
/// │        │        │        │        │
/// │ North  │  East  │ South  │  West  |
/// │        │        │        │        │
/// ├────────┼────────┴────────┴────────┘
/// │        │
/// │ Bottom │
/// │        │
/// └────────┘
///
/// Sides being part of the same band/rotation system as it makes the most sense to me, and North being the first square
/// because I like the idea.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Rotation {
    FacingNorth0Degrees,
    FacingNorth90Degrees,
    FacingNorth180Degrees,
    FacingNorth270Degrees,
    FacingEast0Degrees,
    FacingEast90Degrees,
    FacingEast180Degrees,
    FacingEast270Degrees,
    FacingSouth0Degrees,
    FacingSouth90Degrees,
    FacingSouth180Degrees,
    FacingSouth270Degrees,
    FacingWest0Degrees,
    FacingWest90Degrees,
    FacingWest180Degrees,
    FacingWest270Degrees,
    FacingTop0Degrees,
    FacingTop90Degrees,
    FacingTop180Degrees,
    FacingTop270Degrees,
    FacingBottom0Degrees,
    FacingBottom90Degrees,
    FacingBottom180Degrees,
    FacingBottom270Degrees,
}

impl TryFrom<u8> for Rotation {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        [
            Self::FacingNorth0Degrees,
            Self::FacingNorth90Degrees,
            Self::FacingNorth180Degrees,
            Self::FacingNorth270Degrees,
            Self::FacingEast0Degrees,
            Self::FacingEast90Degrees,
            Self::FacingEast180Degrees,
            Self::FacingEast270Degrees,
            Self::FacingSouth0Degrees,
            Self::FacingSouth90Degrees,
            Self::FacingSouth180Degrees,
            Self::FacingSouth270Degrees,
            Self::FacingWest0Degrees,
            Self::FacingWest90Degrees,
            Self::FacingWest180Degrees,
            Self::FacingWest270Degrees,
            Self::FacingTop0Degrees,
            Self::FacingTop90Degrees,
            Self::FacingTop180Degrees,
            Self::FacingTop270Degrees,
            Self::FacingBottom0Degrees,
            Self::FacingBottom90Degrees,
            Self::FacingBottom180Degrees,
            Self::FacingBottom270Degrees,
        ]
        .get(value as usize)
        .ok_or_else(|| ())
        .map(|v| *v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
    pub rotation: Rotation,
    pub volume: Volume,
}

impl Shape {
    pub const EMPTY: Self = Self {
        rotation: Rotation::FacingNorth0Degrees,
        volume: Volume::ZeroSixth,
    };

    pub const FULL: Self = Self {
        rotation: Rotation::FacingNorth0Degrees,
        volume: Volume::SixSixth,
    };

    pub fn new(rotation: Rotation, volume: Volume) -> Self {
        Self { rotation, volume }
    }

    pub fn to_shape_descriptor(&self) -> BlockDescriptor {
        (self.volume as u8) << 5 | self.rotation as u8
    }

    pub fn to_grid_index(&self) -> GridIndex {
        SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP[self.to_shape_descriptor() as usize]
    }
}

pub static VERTEX_LIST: [UVec3; 8] = [
    UVec3::new(0, 0, 0),
    UVec3::new(1, 0, 0),
    UVec3::new(0, 0, 1),
    UVec3::new(1, 0, 1),
    UVec3::new(0, 1, 0),
    UVec3::new(1, 1, 0),
    UVec3::new(0, 1, 1),
    UVec3::new(1, 1, 1),
];

pub static ZERO_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| vec![]);
static ZERO_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| vec![]);

pub static ONE_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| {
    vec![
        UVec3::new(0, 0, 0),
        UVec3::new(1, 0, 0),
        UVec3::new(0, 0, 1),
        UVec3::new(0, 1, 0),
    ]
});

static ONE_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| {
    vec![[
        UVec3::new(0, 1, 0),
        UVec3::new(0, 0, 1),
        UVec3::new(1, 0, 0),
    ]]
});

pub static TWO_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| {
    vec![
        UVec3::new(0, 0, 0),
        UVec3::new(1, 0, 0),
        UVec3::new(1, 0, 1),
        UVec3::new(0, 0, 1),
        UVec3::new(0, 1, 0),
    ]
});

static TWO_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| {
    vec![
        [
            UVec3::new(0, 1, 0),
            UVec3::new(1, 0, 1),
            UVec3::new(1, 0, 0),
        ],
        [
            UVec3::new(0, 1, 0),
            UVec3::new(0, 0, 1),
            UVec3::new(1, 0, 1),
        ],
    ]
});

pub static THREE_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| {
    vec![
        UVec3::new(0, 0, 0),
        UVec3::new(1, 0, 0),
        UVec3::new(1, 0, 1),
        UVec3::new(0, 0, 1),
        UVec3::new(0, 1, 0),
        UVec3::new(1, 1, 0),
    ]
});

static THREE_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| {
    vec![
        [
            UVec3::new(0, 1, 0),
            UVec3::new(0, 0, 1),
            UVec3::new(1, 0, 1),
        ],
        [
            UVec3::new(0, 1, 0),
            UVec3::new(1, 0, 1),
            UVec3::new(1, 1, 0),
        ],
    ]
});

pub static FOUR_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| {
    vec![
        UVec3::new(0, 0, 0),
        UVec3::new(1, 0, 0),
        UVec3::new(1, 0, 1),
        UVec3::new(0, 0, 1),
        UVec3::new(0, 1, 0),
        UVec3::new(1, 1, 0),
        UVec3::new(0, 1, 1),
    ]
});

static FOUR_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| {
    vec![
        [
            UVec3::new(1, 1, 0),
            UVec3::new(0, 1, 1),
            UVec3::new(0, 0, 1),
        ],
        [
            UVec3::new(1, 1, 0),
            UVec3::new(0, 0, 1),
            UVec3::new(1, 0, 1),
        ],
    ]
});

pub static FIVE_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| {
    vec![
        UVec3::new(0, 0, 0),
        UVec3::new(1, 0, 0),
        UVec3::new(1, 0, 1),
        UVec3::new(0, 0, 1),
        UVec3::new(0, 1, 0),
        UVec3::new(1, 1, 0),
        UVec3::new(0, 1, 1),
    ]
});

static FIVE_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| {
    vec![[
        UVec3::new(1, 1, 0),
        UVec3::new(0, 1, 1),
        UVec3::new(1, 0, 1),
    ]]
});

pub static SIX_SIXTH_VERTEX_LIST: LazyLock<Vec<UVec3>> = LazyLock::new(|| {
    vec![
        UVec3::new(0, 0, 0),
        UVec3::new(1, 0, 0),
        UVec3::new(1, 0, 1),
        UVec3::new(0, 0, 1),
        UVec3::new(0, 1, 0),
        UVec3::new(1, 1, 0),
        UVec3::new(1, 1, 1),
        UVec3::new(0, 1, 1),
    ]
});

static SIX_SIXTH_INTERIOR_VERTICES: LazyLock<Vec<[UVec3; 3]>> = LazyLock::new(|| vec![]);

pub fn vertex_to_index(vertex: UVec3) -> usize {
    VERTEX_LIST.iter().position(|&v| v == vertex).unwrap()
}

pub static SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP: LazyLock<[Vec<[UVec3; 3]>; 256]> =
    LazyLock::new(|| {
        let mut map: [Vec<[UVec3; 3]>; 256] = [(); 256].map(|_| vec![]);
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

        for (shape_index, shape) in [
            &ZERO_SIXTH_INTERIOR_VERTICES,
            &ONE_SIXTH_INTERIOR_VERTICES,
            &TWO_SIXTH_INTERIOR_VERTICES,
            &THREE_SIXTH_INTERIOR_VERTICES,
            &FOUR_SIXTH_INTERIOR_VERTICES,
            &FIVE_SIXTH_INTERIOR_VERTICES,
        ]
        .iter()
        .enumerate()
        {
            for (facing_rotation_index, facing_rotation) in facing_rotations.iter().enumerate() {
                for (face_rotation_index, face_rotation) in face_rotations.iter().enumerate() {
                    let rotation = *facing_rotation + *face_rotation;
                    let rot = Mat4::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
                    let rotated_vertices = shape
                        .iter()
                        .map(|triangle| {
                            triangle
                                .iter()
                                .map(|vertex| {
                                    let center_at_origin =
                                        vertex.as_vec3() - Vec3::new(0.5, 0.5, 0.5);
                                    let rotated = rot.transform_vector3(center_at_origin);
                                    (rotated + Vec3::new(0.5, 0.5, 0.5)).round().as_uvec3()
                                })
                                .collect::<Vec<UVec3>>()
                                .try_into()
                                .unwrap()
                        })
                        .collect::<Vec<[UVec3; 3]>>();

                    let index =
                        (facing_rotation_index * 4 + face_rotation_index) | (shape_index << 5);
                    map[index] = rotated_vertices;
                }
            }
        }

        map
    });

pub static SHAPE_DESCRIPTOR_TO_BLOCK_INDEX_MAP: LazyLock<[BlockIndex; 256]> = LazyLock::new(|| {
    let mut map: [BlockIndex; 256] = [0; 256];
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

    for (shape_index, shape) in [
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
                let rotated_vertices = shape
                    .iter()
                    .map(|vertex| {
                        let center_at_origin = vertex.as_vec3() - Vec3::new(0.5, 0.5, 0.5);
                        let rotated = rot.transform_vector3(center_at_origin);
                        (rotated + Vec3::new(0.5, 0.5, 0.5)).round().as_uvec3()
                    })
                    .collect::<Vec<UVec3>>();
                let block_index = rotated_vertices
                    .iter()
                    .fold(0, |acc, vertex| acc | (1 << vertex_to_index(*vertex)));

                let index = (facing_rotation_index * 4 + face_rotation_index) | (shape_index << 5);
                map[index] = block_index;
            }
        }
    }

    map
});

pub static SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP: LazyLock<[u32; 256]> = LazyLock::new(|| {
    let mut map: [u32; 256] = [1; 256];
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

    for (shape_index, shape) in [
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
        if shape_index == 4 {
            continue;
        } // sinful shape

        for (facing_rotation_index, facing_rotation) in facing_rotations.iter().enumerate() {
            for (face_rotation_index, face_rotation) in face_rotations.iter().enumerate() {
                let rotation = *facing_rotation + *face_rotation;
                let rot = Mat4::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);
                let rotated_vertices = shape
                    .iter()
                    .map(|vertex| {
                        let center_at_origin = vertex.as_vec3() - Vec3::new(0.5, 0.5, 0.5);
                        let rotated = rot.transform_vector3(center_at_origin);
                        (rotated + Vec3::new(0.5, 0.5, 0.5)).round().as_uvec3()
                    })
                    .collect::<Vec<UVec3>>();
                let block_index: BlockIndex = rotated_vertices
                    .iter()
                    .fold(0, |acc, vertex| acc | (1 << vertex_to_index(*vertex)));

                // NORTH
                // 0b1100_1100
                let north_index = block_index & NORTH_FACE_MASK;
                let north_face_flag: u32 = match north_index {
                    0b0100_1100 => 0b0001,
                    0b1000_1100 => 0b0010,
                    0b1100_1000 => 0b0100,
                    0b1100_0100 => 0b1000,
                    0b1100_1100 => 0b1111,
                    _ => 0,
                };

                // EAST
                // 0b1001_1001
                let east_index = block_index & EAST_FACE_MASK;
                let east_face_flag: u32 = match east_index {
                    0b0001_0101 => 0b0001,
                    0b0101_0001 => 0b0010,
                    0b0101_0100 => 0b0100,
                    0b0100_0101 => 0b1000,
                    0b0101_0101 => 0b1111,
                    _ => 0,
                };

                // SOUTH
                // 0b0011_0011
                let south_index = block_index & SOUTH_FACE_MASK;
                let south_face_flag: u32 = match south_index {
                    0b0001_0011 => 0b0001,
                    0b0010_0011 => 0b0010,
                    0b0011_0010 => 0b0100,
                    0b0011_0001 => 0b1000,
                    0b0011_0011 => 0b1111,
                    _ => 0,
                };

                // WEST
                // 0b0110_0110
                let west_index = block_index & WEST_FACE_MASK;
                let west_face_flag: u32 = match west_index {
                    0b0010_1010 => 0b0001,
                    0b1010_0010 => 0b0010,
                    0b1010_1000 => 0b0100,
                    0b1000_1010 => 0b1000,
                    0b1010_1010 => 0b1111,
                    _ => 0,
                };

                // TOP
                // 0b1111_0000
                let top_index = block_index & TOP_FACE_MASK;
                let top_face_flag: u32 = match top_index {
                    0b0111_0000 => 0b0001,
                    0b1101_0000 => 0b0010,
                    0b1110_0000 => 0b0100,
                    0b1011_0000 => 0b1000,
                    0b1111_0000 => 0b1111,
                    _ => 0,
                };

                // BOTTOM
                // 0b0000_1111
                let bottom_index = block_index & BOTTOM_FACE_MASK;
                let bottom_face_flag: u32 = match bottom_index {
                    0b0000_0111 => 0b0001,
                    0b0000_1101 => 0b0010,
                    0b0000_1110 => 0b0100,
                    0b0000_1011 => 0b1000,
                    0b0000_1111 => 0b1111,
                    _ => 0,
                };

                let result: u32 = 0
                    | bottom_face_flag
                    | top_face_flag << 4
                    | west_face_flag << 8
                    | south_face_flag << 12
                    | east_face_flag << 16
                    | north_face_flag << 20;

                let index = (facing_rotation_index * 4 + face_rotation_index) | (shape_index << 5);
                map[index] = result;
            }
        }
    }

    map
});

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
