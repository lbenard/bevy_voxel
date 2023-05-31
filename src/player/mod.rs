use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        info, Camera3d, Camera3dBundle, Color, Commands, FogFalloff, FogSettings, Plugin, Query,
    },
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_mod_raycast::{Intersection, RaycastSource};
use bevy_spectator::{Spectator, SpectatorPlugin, SpectatorSettings};

use crate::world::terrain::{chunk_loader::ChunkLoaderSource, TerrainRaycastSet};

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
            FogSettings {
                color: Color::rgba(0.1, 0.2, 0.4, 1.0),
                falloff: FogFalloff::Linear {
                    start: 200.0,
                    end: 400.0,
                },
                ..Default::default()
            },
            RaycastSource::<TerrainRaycastSet>::new_transform_empty(),
            AtmosphereCamera::default(),
            Spectator,
            ChunkLoaderSource,
        ));
    }

    fn intersection(query: Query<&Intersection<TerrainRaycastSet>>) {
        for intersection in &query {
            info!(
                "Distance {:?}, Position {:?}",
                intersection.distance(),
                intersection.position()
            );
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(SpectatorPlugin)
            .add_startup_system(Self::setup_player)
            .add_system(Self::intersection)
            .insert_resource(SpectatorSettings {
                base_speed: 50.0,
                alt_speed: 2000.0,
                sensitivity: 0.001,
                ..Default::default()
            });
    }
}
