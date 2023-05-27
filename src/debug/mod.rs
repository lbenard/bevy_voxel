use bevy::{prelude::*, render};

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

    fn spawn_sphere(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<render::mesh::Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        pos: UVec3,
    ) {
        let mut material: StandardMaterial = Color::rgb(1.0, 1.0, 1.0).into();
        material.metallic = 0.0;
        material.reflectance = 0.0;

        let icosphere = shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        };

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(icosphere.try_into().unwrap()),
                material: materials.add(material.clone()),
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                ..default()
            })
            .insert(DebugComponent);
    }
}
