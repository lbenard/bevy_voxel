use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{Camera3d, Camera3dBundle, Color, Commands, Plugin},
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_spectator::{Spectator, SpectatorPlugin};

use crate::terrain::chunk_loader::ChunkLoaderSource;

pub struct PlayerPlugin;

impl PlayerPlugin {
    fn setup_player(mut commands: Commands) {
        commands.spawn((
            Camera3dBundle {
                camera_3d: Camera3d {
                    // clear sky color
                    clear_color: ClearColorConfig::Custom(Color::rgb(0.7, 0.8, 1.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            AtmosphereCamera::default(),
            Spectator,
            ChunkLoaderSource,
        ));
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(SpectatorPlugin)
            .add_startup_system(Self::setup_player);
    }
}
