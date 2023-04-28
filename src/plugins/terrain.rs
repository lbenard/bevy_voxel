use std::f32::consts::PI;

use bevy::{prelude::*, render};

use crate::terrain::{
    chunk::{Grid, Mesher},
    generators::noise_terrain::NoiseTerrain,
};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(Self::setup_light);

        #[cfg(debug_assertions)]
        app.add_startup_system(Self::setup_debug_terrain);
        #[cfg(not(debug_assertions))]
        app.add_startup_system(Self::setup_terrain);
    }
}

impl TerrainPlugin {
    fn setup_terrain(
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

    #[cfg(debug_assertions)]
    fn setup_debug_terrain(
        mut commands: Commands,
        mut meshes: ResMut<Assets<render::mesh::Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mut grid = Grid::new(UVec3 { x: 3, y: 3, z: 3 });
        grid.blocks[13] = 0b0001_1111;
        // grid.blocks[1] = 0b1111_1111;

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

    fn setup_light(mut commands: Commands) {
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
}
