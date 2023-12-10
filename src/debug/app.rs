use bevy::{
    app::App,
    ecs::system::Resource,
    reflect::{GetTypeRegistration, Reflect},
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub trait DebugApp {
    fn debug_resource<T: Resource + Reflect + GetTypeRegistration>(&mut self) -> &mut Self;
}

impl DebugApp for App {
    fn debug_resource<T: Resource + Reflect + GetTypeRegistration>(&mut self) -> &mut Self {
        #[cfg(feature = "debug")]
        self.register_type::<T>()
            .add_plugins(ResourceInspectorPlugin::<T>::default());
        self
    }
}
