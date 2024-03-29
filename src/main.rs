#![feature(lazy_cell)]
#![feature(core_intrinsics)]

use bevy::{core::TaskPoolThreadAssignmentPolicy, prelude::*, window::PresentMode};

#[cfg(feature = "taa")]
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(feature = "debug")]
use debug::plugin::DebugPluginBuilder;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use world::{
    chunk::{loader::ChunkLoaderPlugin, material::TerrainMaterial},
    WorldPlugin,
};

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
                    present_mode: PresentMode::Mailbox,
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
            }),
    )
    .add_plugins((
        WorldPlugin,
        ChunkLoaderPlugin::new(1000, 1200),
        MaterialPlugin::<TerrainMaterial>::default(),
        PlayerPlugin,
        EnvironmentPlugin,
        #[cfg(feature = "taa")]
        TemporalAntiAliasPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins((
        DebugPluginBuilder::new().debug_playground().build(),
        WorldInspectorPlugin::new(),
    ));

    app.run();
}
