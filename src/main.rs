#![feature(lazy_cell)]

use bevy::{
    core::TaskPoolThreadAssignmentPolicy,
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{shape::Cube, *},
    render,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use world::terrain::{material::TerrainMaterial, TerrainPlugin};

mod chunk;
mod debug;
mod environment;
mod player;
mod world;

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.add_startup_system(setup_system);

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1920.0, 1080.0).into(),
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            })
            .set(TaskPoolPlugin {
                task_pool_options: TaskPoolOptions {
                    async_compute: TaskPoolThreadAssignmentPolicy {
                        min_threads: 1,
                        max_threads: std::usize::MAX,
                        percent: 1.0,
                    },
                    ..default()
                },
                ..default()
            }),
    )
    .add_plugin(TerrainPlugin)
    .add_plugin(MaterialPlugin::<TerrainMaterial>::default())
    .add_plugin(PlayerPlugin)
    .add_plugin(EnvironmentPlugin::new());

    // #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(DebugPlugin)
        .add_plugin(WorldInspectorPlugin::new());

    app.run();
}

fn setup_system(mut commands: Commands, mut meshes: ResMut<Assets<render::mesh::Mesh>>) {
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_translation(Vec3::new(2.0, 2.0, 4.0))
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    let cube = Cube::new(1.0);

    commands.spawn(PbrBundle {
        mesh: meshes.add(render::mesh::Mesh::from(cube)),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        // add material with texture
        ..default()
    });
}
