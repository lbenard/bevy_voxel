use bevy::{
    core_pipeline::{clear_color::ClearColorConfig, experimental::taa::TemporalAntiAliasBundle},
    pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings},
    prelude::{
        default, Camera, Camera3d, Camera3dBundle, Color, Commands, FogFalloff, FogSettings,
        PerspectiveProjection, Plugin, Projection, Startup,
    },
};
#[cfg(feature = "atmosphere")]
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_spectator::{Spectator, SpectatorPlugin, SpectatorSettings};

use crate::world::chunk::loader::ChunkLoaderSource;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(SpectatorPlugin)
            .add_systems(Startup, Self::setup_player)
            .insert_resource(SpectatorSettings {
                base_speed: 50.0,
                alt_speed: 2000.0,
                sensitivity: 0.001,
                ..Default::default()
            });
    }
}

impl PlayerPlugin {
    fn setup_player(mut commands: Commands) {
        commands
            .spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..default()
                    },
                    camera_3d: Camera3d {
                        // clear sky color
                        clear_color: ClearColorConfig::Custom(Color::rgb(0.7, 0.8, 1.0)),
                        ..default()
                    },
                    projection: Projection::Perspective(PerspectiveProjection {
                        far: 5000.0,
                        ..default()
                    }),
                    ..default()
                },
                FogSettings {
                    color: Color::rgba(0.1, 0.2, 0.4, 1.0),
                    falloff: FogFalloff::Linear {
                        start: 500.0,
                        end: 1000.0,
                    },
                    ..default()
                },
                #[cfg(feature = "atmosphere")]
                AtmosphereCamera::default(),
                Spectator,
                ChunkLoaderSource,
            ))
            .insert(ScreenSpaceAmbientOcclusionBundle {
                settings: ScreenSpaceAmbientOcclusionSettings {
                    quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Custom {
                        slice_count: 3,
                        samples_per_slice_side: 3,
                    },
                },
                ..default()
            })
            .insert(TemporalAntiAliasBundle::default());
    }

    // fn raycast(camera: Query<(&Camera, &Transform)>, world: Res<World>) {
    //     for (camera, transform) in &camera {
    //         // rotation to direction vector
    //         let direction = Vec3::new(
    //             -transform.rotation.x.sin() * transform.rotation.y.cos(),
    //             transform.rotation.x.cos(),
    //             -transform.rotation.x.sin() * transform.rotation.y.sin(),
    //         );
    //         let result = raycast(transform.translation, direction, 100.0, &world);
    //         if let Some(result) = result {
    //             info!("Raycast hit {:?}", result);
    //         }
    //     }
    // }
}
