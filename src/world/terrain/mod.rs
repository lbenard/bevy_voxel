pub mod block_descriptor;
pub mod chunk_loader;
pub mod generator;
pub mod material;

use bevy::prelude::*;

use self::chunk_loader::ChunkLoaderPlugin;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ChunkLoaderPlugin::new(8, 9));
    }
}
