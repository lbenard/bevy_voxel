use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::world::voxel::{
    material::*,
    shape::{Shape, Volume},
    VoxelDescriptor,
};

use super::Raycast;

pub struct BuildPlugin;

impl Plugin for BuildPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                Self::interact.run_if(input_just_pressed(MouseButton::Left)),
                Self::place.run_if(input_just_pressed(MouseButton::Right)),
            ),
        );
    }
}

impl BuildPlugin {
    fn interact(world: Res<crate::world::World>, raycast: Res<Raycast>) {
        let Some(result) = raycast.result else { return };
        let Some(chunk) = world.get_chunk_at_pos(result.position) else { return };
        {
            let mut chunk_lock = chunk.write();
            let relative_position = chunk_lock.get_relative_position(result.position);
            let Some(ref mut terrain) = chunk_lock.terrain else { return };
            let Some(voxel) = terrain.voxel_at_pos_mut(relative_position.as_ivec3()) else { return };
            voxel.shape.volume = Volume::ZeroSixth;
        }
        chunk.write().dirty = true;
    }

    fn place(world: Res<crate::world::World>, raycast: Res<Raycast>) {
        let Some(result) = raycast.result else { return };
        let Some(chunk) = world.get_chunk_at_pos(result.position) else { return };
        {
            let mut chunk_lock = chunk.write();
            let relative_position = chunk_lock.get_relative_position(result.position);
            let Some(ref mut terrain) = chunk_lock.terrain else { return };
            let voxel =
                terrain.voxel_at_pos_mut(relative_position.as_ivec3() + IVec3::new(0, 1, 0));
            *voxel = Some(VoxelDescriptor {
                shape: Shape::FULL,
                material: GRASS,
            });
        }
        chunk.write().dirty = true;
    }
}
