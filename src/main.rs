#![feature(lazy_cell)]

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};
use bevy_inspector_egui::{bevy_egui, quick::WorldInspectorPlugin, DefaultInspectorConfigPlugin};
use plugins::{debug::DebugPlugin, player::PlayerPlugin, terrain::TerrainPlugin};
use terrain::block::{Rotation, Shape, Volume};

mod plugins;
mod terrain;

#[bevy_main]
fn main() {
    let shape = Shape::new(Rotation::FacingNorth0Degrees, Volume::SixSixth).to_shape_descriptor();
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            present_mode: PresentMode::Immediate,
            ..default()
        },
        ..default()
    }));
    app.add_plugin(TerrainPlugin).add_plugin(PlayerPlugin);

    // #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DebugPlugin)
        .add_plugin(WorldInspectorPlugin);

    app.run();
}
