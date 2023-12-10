use std::f32::consts::PI;

#[cfg(feature = "atmosphere")]
use bevy::prelude::Vec3;
#[cfg(feature = "atmosphere")]
use bevy_atmosphere::{
    prelude::{AtmosphereModel, AtmospherePlugin, Nishita},
    system_param::AtmosphereMut,
};

#[cfg(not(feature = "atmosphere"))]
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{Camera3d, FogSettings},
};

use bevy::{
    prelude::{
        default, AmbientLight, Color, Commands, Component, DirectionalLight,
        DirectionalLightBundle, Plugin, PointLight, PointLightBundle, Quat, Query, ReflectResource,
        Res, ResMut, Resource, Startup, Transform, Update, With, Without,
    },
    reflect::Reflect,
    time::{Time, Timer, TimerMode},
};
use bevy_spectator::Spectator;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

/// Daylight value between 0.0 and 1.0 (0.0 = night, 1.0 = day)
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct DaylightCycle(f32);

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[cfg(feature = "atmosphere")]
        {
            app.insert_resource(AtmosphereModel::default());
            app.add_plugins(AtmospherePlugin)
                .add_systems(Update, Self::update_atmosphere);
        }
        #[cfg(not(feature = "atmosphere"))]
        app.add_systems(Update, Self::update_clear_color);

        app.add_systems(Startup, Self::setup_player_light)
            .add_systems(Update, Self::update_player_light);

        app.insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(16), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
            TimerMode::Repeating,
        )))
        .init_resource::<DaylightCycle>()
        .add_systems(Startup, Self::setup_environment)
        .add_systems(Update, (Self::daylight_cycle, Self::update_lights));
    }
}

impl EnvironmentPlugin {
    fn setup_environment(mut commands: Commands) {
        commands.spawn((
            Sun,
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 10_000.0,
                    // shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_euler(
                    bevy::prelude::EulerRot::ZYX,
                    0.0,
                    PI * -0.15,
                    PI * -0.15,
                )),
                ..default()
            },
        ));
        commands.insert_resource(AmbientLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            brightness: 1.0,
        });
    }

    fn setup_player_light(mut commands: Commands) {
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                range: 100.0,
                ..default()
            },
            ..default()
        });
    }

    fn update_player_light(
        player: Query<(&Transform,), With<Spectator>>,
        mut point_light: Query<(&mut Transform,), (With<PointLight>, Without<Spectator>)>,
    ) {
        let player = player.single();
        let mut point_light = point_light.single_mut();

        point_light.0.translation = player.0.translation;
    }

    fn daylight_cycle(
        mut timer: ResMut<CycleTimer>,
        time: Res<Time>,
        mut daylight: ResMut<DaylightCycle>,
    ) {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let t = time.elapsed_seconds_wrapped() / 100.0;
            daylight.0 = (t.sin() + 1.0) / 2.0;
        }
    }

    #[cfg(not(feature = "atmosphere"))]
    fn update_clear_color(
        daylight: Res<DaylightCycle>,
        mut camera_3d: Query<(&mut Camera3d,)>,
        mut fog: Query<(&mut FogSettings,)>,
    ) {
        camera_3d.single_mut().0.clear_color = ClearColorConfig::Custom(Color::rgb(
            0.7 * daylight.0,
            0.8 * daylight.0,
            1.0 * daylight.0,
        ));
        fog.single_mut().0.color = Color::rgb(0.7 * daylight.0, 0.8 * daylight.0, 1.0 * daylight.0);
    }

    #[cfg(feature = "atmosphere")]
    fn update_atmosphere(
        mut atmosphere: AtmosphereMut<Nishita>,
        timer: Res<CycleTimer>,
        time: Res<Time>,
    ) {
        if timer.0.finished() {
            let t = time.elapsed_seconds_wrapped() as f32 / 100.0;
            atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());
        }
    }

    fn update_lights(
        mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
        timer: Res<CycleTimer>,
        daylight: Res<DaylightCycle>,
        mut ambient_light: ResMut<AmbientLight>,
        time: Res<Time>,
    ) {
        if timer.0.finished() {
            let t = time.elapsed_seconds_wrapped() / 100.0;

            if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
                light_trans.rotation = Quat::from_rotation_x(-t.sin().atan2(t.cos()));
                directional.illuminance = interpolation::lerp(&0.0, &100_000.0, &daylight.0);
            }
            ambient_light.brightness = interpolation::lerp(&0.1, &1.0, &daylight.0);
        }
    }
}
