pub mod block_descriptor;
pub mod chunk_loader;
pub mod generator;
pub mod material;

use bevy::prelude::*;

use self::chunk_loader::ChunkLoaderPlugin;

pub struct TerrainPlugin;

// #[derive(Reflect, Clone)]
// pub struct TerrainRaycastSet;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ChunkLoaderPlugin::new(8, 9));
        // .add_plugin(DefaultRaycastingPlugin::<TerrainRaycastSet>::default())
        // .insert_resource(
        //     DefaultPluginState::<TerrainRaycastSet>::default().with_debug_cursor(),
        // );
    }
}
