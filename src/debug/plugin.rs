use std::time::Duration;

use bevy::{pbr::ExtendedMaterial, prelude::*, render};

use crate::world::chunk::material::{
    StandardMaterialExtension, TerrainMaterial, ATTRIBUTE_VOXEL_ID,
};

#[derive(Component)]
pub struct DebugComponent;

#[derive(Default)]
pub struct DebugPluginBuilder {
    debug_playground: bool,
    adhd_autoclose: Option<Duration>,
}

impl DebugPluginBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DebugPlugin {
        DebugPlugin {
            debug_playground: self.debug_playground,
            adhd_autoclose: self.adhd_autoclose,
        }
    }

    #[allow(dead_code)]
    pub fn debug_playground(mut self) -> Self {
        self.debug_playground = true;
        self
    }

    #[allow(dead_code)]
    pub fn with_adhd_autoclose(mut self, duration: Duration) -> Self {
        self.adhd_autoclose = Some(duration);
        self
    }
}

#[derive(Resource)]
struct AutoClose(Timer);

pub struct DebugPlugin {
    debug_playground: bool,
    adhd_autoclose: Option<Duration>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.debug_playground {
            app.add_systems(Startup, Self::debug_playground);
        }
        if let Some(duration) = self.adhd_autoclose {
            app.insert_resource(AutoClose(Timer::new(duration, TimerMode::Once)))
                .add_systems(Update, Self::auto_close);
        }
    }
}

impl DebugPlugin {
    fn debug_playground(
        mut commands: Commands,
        mut meshes: ResMut<Assets<render::mesh::Mesh>>,
        mut standard_material: ResMut<Assets<StandardMaterial>>,
        mut terrain_material: ResMut<Assets<TerrainMaterial>>,
    ) {
        Self::spawn_sphere(
            &mut commands,
            &mut meshes,
            &mut standard_material,
            UVec3 { x: 0, y: 0, z: 0 },
        );

        let mesh = Mesh::from(shape::Cube { size: 1.0 }).with_inserted_attribute(
            ATTRIBUTE_VOXEL_ID,
            vec![
                1_u32, 1_u32, 1_u32, 1_u32, 2_u32, 2_u32, 2_u32, 2_u32, 3_u32, 3_u32, 3_u32, 3_u32,
                3_u32, 3_u32, 3_u32, 3_u32, 2_u32, 2_u32, 2_u32, 2_u32, 1_u32, 1_u32, 1_u32, 1_u32,
            ],
        );

        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: terrain_material.add(ExtendedMaterial {
                base: StandardMaterial::default(),
                extension: StandardMaterialExtension {},
            }),
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

        let icosphere = shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        };

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(icosphere.try_into().unwrap()),
                material: materials.add(material),
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, pos.z as f32),
                ..default()
            })
            .insert(DebugComponent);
    }

    fn auto_close(mut timer: ResMut<AutoClose>, time: Res<Time>) {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            std::process::exit(0);
        }
    }
}
