pub mod block;
pub mod chunk_loader;
mod chunk_mesher;
pub mod generator;

use bevy::prelude::*;

use self::chunk_loader::ChunkLoaderPlugin;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(ChunkLoaderPlugin::new(4));
    }
}
