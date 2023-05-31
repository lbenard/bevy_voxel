#![feature(lazy_cell)]

use std::time::Duration;

use bevy::{
    core::TaskPoolThreadAssignmentPolicy, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use debug::DebugPluginBuilder;
use environment::EnvironmentPlugin;
use player::PlayerPlugin;
use world::{
    terrain::{material::TerrainMaterial, TerrainPlugin},
    WorldPlugin,
};

mod chunk;
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
    .add_plugin(WorldPlugin)
    .add_plugin(TerrainPlugin)
    .add_plugin(MaterialPlugin::<TerrainMaterial>::default())
    .add_plugin(PlayerPlugin)
    .add_plugin(EnvironmentPlugin::new());

    #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(
            DebugPluginBuilder::new()
                .with_adhd_autoclose(Duration::from_secs(10))
                .build(),
        )
        .add_plugin(WorldInspectorPlugin::new());

    app.run();
}
