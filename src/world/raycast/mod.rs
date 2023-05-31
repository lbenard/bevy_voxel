use bevy::prelude::Vec3;

use crate::chunk::Grid;

use super::terrain::block_descriptor::BlockDescriptor;

pub struct RaycastResult {
    pub position: Vec3,
    pub face: Vec3,
    pub block: BlockDescriptor,
}

pub fn raycast(origin: Vec3, direction: Vec3, radius: f32, grid: &Grid) -> Option<RaycastResult> {
    None
}
