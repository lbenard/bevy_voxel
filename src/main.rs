#![feature(lazy_cell)]
#![feature(core_intrinsics)]

use bevy::{
    core::TaskPoolThreadAssignmentPolicy,
    core_pipeline::experimental::taa::TemporalAntiAliasPlugin, prelude::*, window::PresentMode,
};

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(debug_assertions)]
use debug::DebugPluginBuilder;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use world::{
    chunk::{loader::ChunkLoaderPlugin, material::TerrainMaterial, CHUNK_LENGTH},
    WorldPlugin,
};

#[cfg(debug_assertions)]
mod debug;
mod environment;
mod player;
mod world;

#[bevy_main]
fn main() {
    let mut app = App::new();

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
    .add_plugins((
        WorldPlugin,
        ChunkLoaderPlugin::new(CHUNK_LENGTH / 4, CHUNK_LENGTH / 2),
        MaterialPlugin::<TerrainMaterial>::default(),
        PlayerPlugin,
        EnvironmentPlugin,
        TemporalAntiAliasPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        DebugPluginBuilder::new().debug_sphere_at_origin().build(),
        WorldInspectorPlugin::new(),
    ));

    app.run();
}
