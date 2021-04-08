use bevy::prelude::*;

mod generate_map;
mod spawn_map_entity;

use spawn_map_entity::TilemapAtlasHandles;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut tilemap_atlas_handles: ResMut<TilemapAtlasHandles>,
) {
    tilemap_atlas_handles.handles = asset_server.load_folder("textures").unwrap();
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<TilemapAtlasHandles>()
            .add_startup_system(setup.system())
            .add_system(spawn_map_entity::spawn_map_entity.system())
            .add_system(generate_map::generate_map.system());
    }
}