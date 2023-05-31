use std::time::Duration;

use bevy::{prelude::*, render};

#[derive(Component)]
pub struct DebugComponent;

#[derive(Default)]
pub struct DebugPluginBuilder {
    debug_sphere_at_origin: bool,
    adhd_autoclose: Option<Duration>,
}

impl DebugPluginBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DebugPlugin {
        DebugPlugin {
            debug_sphere_at_origin: self.debug_sphere_at_origin,
            adhd_autoclose: self.adhd_autoclose,
        }
    }

    #[allow(dead_code)]
    pub fn debug_sphere_at_origin(mut self) -> Self {
        self.debug_sphere_at_origin = true;
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
    debug_sphere_at_origin: bool,
    adhd_autoclose: Option<Duration>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if self.debug_sphere_at_origin {
            app.add_startup_system(Self::origin_sphere);
        }
        if let Some(duration) = self.adhd_autoclose {
            app.insert_resource(AutoClose(Timer::new(duration, TimerMode::Once)))
                .add_system(Self::auto_close);
        }
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

    fn auto_close(mut timer: ResMut<AutoClose>, time: Res<Time>) {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            std::process::exit(0);
        }
    }
}
