#[cfg(feature = "taa")]
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
#[cfg(feature = "ssao")]
use bevy::pbr::{ScreenSpaceAmbientOcclusionBundle, ScreenSpaceAmbientOcclusionSettings};
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
#[cfg(feature = "atmosphere")]
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_spectator::{Spectator, SpectatorPlugin, SpectatorSettings};

use crate::world::{chunk::loader::ChunkLoaderSource, raycast::fast_voxel_traversal, World};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(SpectatorPlugin)
            .add_systems(Startup, Self::setup_player)
            .add_systems(Update, Self::raycast)
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
        commands.spawn((
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
        ));
        #[cfg(feature = "ssao")]
        commands.spawn(ScreenSpaceAmbientOcclusionBundle {
            settings: ScreenSpaceAmbientOcclusionSettings {
                quality_level: bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel::Custom {
                    slice_count: 3,
                    samples_per_slice_side: 3,
                },
            },
            ..default()
        });
        #[cfg(feature = "taa")]
        commands.spawn(TemporalAntiAliasBundle::default());
    }

    fn raycast(mut gizmos: Gizmos, camera: Query<(&Camera, &Transform)>, world: Res<World>) {
        for (_camera, transform) in &camera {
            let direction = transform.forward();
            let result = fast_voxel_traversal(&world, transform.translation, 100.0, direction);
            if let Some(result) = result {
                gizmos.cuboid(
                    Transform::from_translation(result.0.as_vec3() + Vec3::new(0.5, 0.5, 0.5))
                        .with_scale(Vec3::splat(1.)),
                    Color::BLACK,
                );
            }
        }
    }
}
