use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        AmbientLight, Camera3d, ClearColor, Color, Commands, Component, DirectionalLight,
        DirectionalLightBundle, Plugin, Quat, Query, Res, ResMut, Resource, Transform, Vec3, With,
    },
    render::camera,
    time::{Time, Timer, TimerMode},
};
use bevy_atmosphere::{
    prelude::{AtmosphereModel, AtmospherePlugin, Nishita},
    system_param::AtmosphereMut,
};

pub struct EnvironmentPlugin {
    with_atmosphere: bool,
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
struct CycleTimer(Timer);

/// Daylight value between 0.0 and 1.0 (0.0 = night, 1.0 = day)
#[derive(Resource, Default)]
struct DaylightCycle(f32);

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(AtmosphereModel::default())
            .insert_resource(CycleTimer(Timer::new(
                bevy::utils::Duration::from_millis(16), // Update our atmosphere every 50ms (in a real game, this would be much slower, but for the sake of an example we use a faster update)
                TimerMode::Repeating,
            )))
            .init_resource::<DaylightCycle>()
            .add_startup_system(Self::setup_environment)
            .add_system(Self::daylight_cycle)
            .add_system(Self::update_lights);
        if self.with_atmosphere {
            app.add_plugin(AtmospherePlugin)
                .add_system(Self::update_atmosphere);
        } else {
            app.add_system(Self::update_clear_color);
        }
    }
}

impl EnvironmentPlugin {
    pub fn new() -> Self {
        Self {
            with_atmosphere: false,
        }
    }

    pub fn with_atmosphere() -> Self {
        Self {
            with_atmosphere: true,
        }
    }

    fn setup_environment(mut commands: Commands) {
        commands.spawn((Sun, DirectionalLightBundle::default()));
        commands.insert_resource(AmbientLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            brightness: 1.0,
        });
    }

    fn daylight_cycle(
        mut timer: ResMut<CycleTimer>,
        time: Res<Time>,
        mut daylight: ResMut<DaylightCycle>,
    ) {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            let t = time.elapsed_seconds_wrapped() as f32 / 100.0;
            daylight.0 = (t.sin() + 1.0) / 2.0;
        }
    }

    fn update_clear_color(daylight: Res<DaylightCycle>, mut camera_3d: Query<(&mut Camera3d,)>) {
        camera_3d.single_mut().0.clear_color = ClearColorConfig::Custom(Color::rgb(
            0.7 * daylight.0,
            0.8 * daylight.0,
            1.0 * daylight.0,
        ));
    }

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
            let t = time.elapsed_seconds_wrapped() as f32 / 100.0;

            if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
                light_trans.rotation = Quat::from_rotation_x(-t.sin().atan2(t.cos()));
                directional.illuminance = (daylight.0.powf(2.0) * 100_000.0).max(10_000.0);
            }
            ambient_light.brightness = interpolation::lerp(&0.5, &1.0, &daylight.0);
        }
    }
}