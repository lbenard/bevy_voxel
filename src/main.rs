#![feature(lazy_cell)]

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use bevy_inspector_egui::WorldInspectorPlugin;
use plugins::{debug::DebugPlugin, player::PlayerPlugin, terrain::TerrainPlugin};
use terrain::block::{Rotation, Shape, Volume};

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
        .add_plugin(TerrainPlugin)
        .add_plugin(PlayerPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(DebugPlugin)
        .add_plugin(WorldInspectorPlugin::new());

    app.run();
}
