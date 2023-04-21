#![feature(once_cell)] // 1.65.0-nightly

use std::f32::consts::PI;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render,
    window::PresentMode,
};
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use terrain::{
    block::{Rotation, Shape, Volume, SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP},
    chunk::{Grid, Mesher},
    generators::noise_terrain::NoiseTerrain,
    marching_cube::INDEX_LOOKUP,
};

use crate::terrain::block::SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP;

mod terrain;

#[bevy_main]
fn main() {
    let shape = Shape::new(Rotation::FacingNorth0Degrees, Volume::SixSixth).to_shape_descriptor();
    // println!("shape descriptor {:#10b} {}", shape, shape);
    // println!("{:#18b}", SHAPE_DESCRIPTOR_TO_FACE_FLAGS_MAP[shape as usize]);
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: 1920.0,
        height: 1080.0,
        present_mode: PresentMode::Immediate,
        ..default()
    });
    app.add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(AtmospherePlugin)
        .add_startup_system(env_setup)
        .add_startup_system(terrain_setup)
        .add_startup_system(debug_setup);

    #[cfg(debug_assertions)]
    app.add_system(bevy::window::close_on_esc);
    app.add_system(index_debug);

    app.run();
}

#[derive(Component)]
struct DebugComponent;

fn debug_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<render::mesh::Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesher = Mesher::new();

    let mut material: StandardMaterial = Color::rgb(0.0, 0.6, 0.1).into();
    material.metallic = 0.0;
    material.reflectance = 0.0;

    // for (volume_index, volume) in [
    //     Volume::OneSixth,
    //     Volume::TwoSixth,
    //     Volume::ThreeSixth,
    //     Volume::FourSixth,
    //     Volume::FiveSixth,
    // ]
    // .iter()
    // .enumerate()
    // {
    //     for (rotation_index, rotation) in [
    //         Rotation::FacingNorth0Degrees,
    //         Rotation::FacingEast0Degrees,
    //         Rotation::FacingSouth0Degrees,
    //         Rotation::FacingWest0Degrees,
    //     ]
    //     .iter()
    //     .enumerate()
    //     {
    //         mesher.add_vertices_at_pos(
    //             &SHAPE_DESCRIPTOR_TO_INTERIOR_VERTICES_MAP
    //                 [Shape::new(*rotation, *volume).to_shape_descriptor() as usize],
    //             UVec3 {
    //                 x: rotation_index as u32,
    //                 y: 0,
    //                 z: volume_index as u32 * 2,
    //             },
    //         );

    //         // debug sphere magic
    //         commands
    //             .spawn_bundle(PbrBundle {
    //                 mesh: meshes.add(render::mesh::Mesh::from(shape::Icosphere {
    //                     radius: 0.2,
    //                     subdivisions: 1,
    //                 })),
    //                 material: materials.add(material.clone()),
    //                 transform: Transform::from_xyz(
    //                     rotation_index as f32,
    //                     0.0,
    //                     volume_index as f32 * 2.0,
    //                 ),
    //                 ..default()
    //             })
    //             .insert(DebugComponent);
    //     }
    // }

    // let mesh = mesher.mesh();
    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(mesh),
    //     material: materials.add(material.clone()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // });
}

fn terrain_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<render::mesh::Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // terrain
    let generator = NoiseTerrain::new();
    // let generator = FlatTerrain::new();
    let grid = Grid::new(UVec3 {
        x: 128,
        y: 128,
        z: 128,
    })
    .generate(&generator);

    let mesh = Mesher::new().mesh_grid(&grid).mesh();
    // println!(
    //     "{:?}",
    //     INDEX_LOOKUP[grid.index_at_pos(UVec3 { x: 7, y: 9, z: 1 }) as usize]
    // );
    // println!("{:#10b}", grid.index_at_pos(UVec3 { x: 7, y: 34, z: 0 }));
    // println!(
    //     "{:#18b}",
    //     Mesh::lookup_idx(&grid, UVec3 { x: 7, y: 9, z: 1 })
    // );
    let mut material: StandardMaterial = Color::rgb(0.0, 0.6, 0.1).into();
    material.metallic = 0.0;
    material.reflectance = 0.0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(material.clone()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // panic!();

    // debug
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(render::mesh::Mesh::from(shape::Icosphere {
                radius: 0.2,
                subdivisions: 1,
            })),
            material: materials.add(material.clone()),
            transform: Transform::from_xyz(25.0, 44.0, 3.0),
            ..default()
        })
        .insert(DebugComponent);
    commands
    .spawn_bundle(PbrBundle {
        mesh: meshes.add(render::mesh::Mesh::from(shape::Icosphere {
            radius: 0.2,
            subdivisions: 1,
        })),
        material: materials.add(material.clone()),
        transform: Transform::from_xyz(21.0, 45.0, 0.0),
        ..default()
    })
    .insert(DebugComponent);
}

fn env_setup(mut commands: Commands) {
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 60_000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(-PI / 3.0)),
        ..default()
    });
}

fn index_debug(debug: Query<(&Transform, &DebugComponent)>) {
    for (_transform, _) in debug.iter() {
        // println!()
    }
}
