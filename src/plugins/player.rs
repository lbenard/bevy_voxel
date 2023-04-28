use bevy::prelude::Plugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(bevy_flycam::PlayerPlugin);
    }
}
