#![feature(lazy_cell)]

use std::f32::consts::PI;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render,
    window::PresentMode,
};
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use plugins::debug::DebugPlugin;
use terrain::{
    block::{Rotation, Shape, Volume},
    chunk::{Grid, Mesher},
    generators::noise_terrain::NoiseTerrain,
};

mod plugins;
mod terrain;

#[bevy_main]
fn main() {
    let shape = Shape::new(Rotation::FacingNorth0Degrees, Volume::SixSixth).to_shape_descriptor();
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
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(AtmospherePlugin)
        .add_startup_system(env_setup)
        .add_startup_system(terrain_setup);

    #[cfg(debug_assertions)]
    app
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DebugPlugin);

    app.run();
}

fn terrain_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<render::mesh::Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let generator = NoiseTerrain::new();
    let grid = Grid::new(UVec3 {
        x: 128,
        y: 128,
        z: 128,
    })
    .generate(&generator);

    let mesh = Mesher::new().mesh_grid(&grid).mesh();
    let mut material: StandardMaterial = Color::rgb(0.0, 0.6, 0.1).into();
    material.metallic = 0.0;
    material.reflectance = 0.0;
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(material.clone()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
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

// fn index_debug(debug: Query<(&Transform, &DebugComponent)>) {
//     for (_transform, _) in debug.iter() {
//         // println!()
//     }
// }
