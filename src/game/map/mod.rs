use bevy::prelude::*;

mod generate_map;
mod get_has_map_assets;
mod map;
mod spawn_map_entity;

pub use self::map::MapData;
pub use get_has_map_assets::get_has_map_assets;
pub use spawn_map_entity::TilemapAtlasHandles;

pub use generate_map::{generate_map, generate_road};
pub use spawn_map_entity::spawn_map_entity;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut tilemap_atlas_handles: ResMut<TilemapAtlasHandles>,
) {
    // asset_server.load_folder("textures").unwrap();
    tilemap_atlas_handles.handles = asset_server.load_folder("textures/map").unwrap();
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TilemapAtlasHandles>()
            .init_resource::<MapData>()
            .add_startup_system(setup.system());
    }
}
