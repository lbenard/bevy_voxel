use bevy::prelude::Vec3;

use super::{terrain::block_descriptor::BlockDescriptor, World};

#[derive(Debug)]
pub struct RaycastResult {
    pub position: Vec3,
    // pub face: Vec3,
    pub block: BlockDescriptor,
}

pub fn raycast(origin: Vec3, direction: Vec3, radius: f32, world: &World) -> Option<RaycastResult> {
    let sign = direction.signum();
    let reciprocal = 1.0 / direction.abs();

    let mut position = origin.floor();
    let mut steps = (sign + 1.0) / 2.0 - (origin - position) / direction;

    loop {
        let axis = argmin(steps);
        position[axis] += sign[axis];
        if origin.distance(position) > radius {
            break;
        }
        if let Some(block) = world.get_block(position.as_ivec3()) {
            // let face = Vec3::from_element(axis as f32) * sign[axis];
            return Some(RaycastResult {
                position: origin + direction * steps[axis],
                // face,
                block: block.into(),
            });
        }
        steps[axis] += reciprocal[axis];
    }
    None
}

fn argmin(v: Vec3) -> usize {
    v.to_array()
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap()
        .0
}
