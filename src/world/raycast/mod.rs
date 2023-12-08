use bevy::math::{IVec3, Vec3};

use crate::world::voxel::{shape::Volume, Side};

use super::World;

pub fn cast(
    world: &World,
    origin: Vec3,
    max_radius: f32,
    direction: Vec3,
) -> Option<(IVec3, Side)> {
    let step = 0.01;

    let mut current_pos = origin;
    let mut distance_traveled = 0.0;

    while distance_traveled <= max_radius {
        let voxel_pos = IVec3::new(
            current_pos.x.floor() as i32,
            current_pos.y.floor() as i32,
            current_pos.z.floor() as i32,
        );

        if let Some(voxel) = world.get_voxel(voxel_pos) {
            if voxel.shape.volume != Volume::ZeroSixth {
                let face = if direction.x > 0.0 {
                    Side::West
                } else if direction.x < 0.0 {
                    Side::East
                } else if direction.y > 0.0 {
                    Side::Bottom
                } else if direction.y < 0.0 {
                    Side::Top
                } else if direction.z > 0.0 {
                    Side::South
                } else {
                    Side::North
                };

                return Some((voxel_pos, face));
            }
        }

        current_pos.x += direction.x * step;
        current_pos.y += direction.y * step;
        current_pos.z += direction.z * step;

        distance_traveled += step;
    }

    None
}
