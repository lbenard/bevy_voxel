use bevy::{prelude::*, render};

use crate::terrain::{
    block::{Rotation, Shape, Volume, SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP},
    chunk::{Grid, Mesher},
};

#[derive(Component)]
pub struct DebugComponent;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::origin_sphere);
    }
}

impl DebugPlugin {
    fn origin_sphere(
        mut commands: Commands,
        mut meshes: ResMut<Assets<render::mesh::Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        Self::spawn_sphere(
            &mut commands,
            &mut meshes,
            &mut materials,
            UVec3 { x: 0, y: 0, z: 0 },
        );
    }

    fn shape_debug(
        mut commands: Commands,
        mut meshes: ResMut<Assets<render::mesh::Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut mesher = Mesher::new();

        let mut material: StandardMaterial = Color::rgb(0.0, 0.6, 0.1).into();
        material.metallic = 0.0;
        material.reflectance = 0.0;

        for (volume_index, volume) in [
            Volume::OneSixth,
            Volume::TwoSixth,
            Volume::ThreeSixth,
            Volume::FourSixth,
            Volume::FiveSixth,
        ]
        .iter()
        .enumerate()
        {
            for (rotation_index, rotation) in [
                Rotation::FacingNorth0Degrees,
                Rotation::FacingEast0Degrees,
                Rotation::FacingSouth0Degrees,
                Rotation::FacingWest0Degrees,
            ]
            .iter()
            .enumerate()
            {
                mesher.add_vertices_at_pos(
                    &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP
                        [Shape::new(*rotation, *volume).to_shape_descriptor() as usize],
                    UVec3 {
                        x: rotation_index as u32,
                        y: 0,
                        z: volume_index as u32 * 2,
                    },
                );

                // debug sphere magic
                commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(render::mesh::Mesh::from(shape::Icosphere {
                            radius: 0.2,
                            subdivisions: 1,
                        })),
                        material: materials.add(material.clone()),
                        transform: Transform::from_xyz(
                            rotation_index as f32,
                            0.0,
                            volume_index as f32 * 2.0,
                        ),
                        ..default()
                    })
                    .insert(DebugComponent);
            }
        }

        let mesh = mesher.mesh();
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material.clone()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });
    }

    fn spawn_sphere(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<render::mesh::Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        pos: UVec3,
    ) {
        let mut material: StandardMaterial = Color::rgb(1.0, 1.0, 1.0).into();
        material.metallic = 0.0;
        material.reflectance = 0.0;

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(render::mesh::Mesh::from(shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 1,
                })),
                material: materials.add(material.clone()),
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                ..default()
            })
            .insert(DebugComponent);
    }
}
